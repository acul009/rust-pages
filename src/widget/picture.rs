use crate::{style::Class, widget::Widget};
use anyhow::Context;
use image::{DynamicImage, ImageReader, imageops::FilterType};
use rayon::prelude::*;
use std::{
    borrow::Cow,
    collections::{BTreeMap, hash_map::DefaultHasher},
    fmt::{Display, Write},
    fs::{self, File},
    hash::{Hash, Hasher},
    io::Cursor,
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

pub struct Handle {
    versions: Vec<PictureVersion>,
    fallback: PictureVersion,
}

impl Handle {
    pub fn create(original: &str) -> Result<Self, anyhow::Error> {
        let source = PathBuf::from(original);
        let canonical = source
            .canonicalize()
            .with_context(|| format!("Failed to resolve picture at {}", source.display()))?;
        let metadata = fs::metadata(&canonical).with_context(|| {
            format!(
                "Failed to read picture metadata for {}",
                canonical.display()
            )
        })?;
        if !metadata.is_file() {
            anyhow::bail!("Picture source is not a file: {}", canonical.display());
        }

        let source_image = SourceImage::load(&canonical)?;
        let variants = build_variants(&canonical, &source_image);
        let fallback = variants
            .iter()
            .rev()
            .find(|variant| variant.format == ImageFormat::Jpeg)
            .cloned()
            .context("No JPEG fallback generated for picture")?;

        generate_variants(&source_image, &variants)?;

        Ok(Self {
            versions: variants,
            fallback,
        })
    }
}

#[derive(Clone)]
struct PictureVersion {
    resolution: (u32, u32),
    format: ImageFormat,
    location: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ImageFormat {
    WebP,
    Avif,
    Jpeg,
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageFormat::WebP => write!(f, "image/webp"),
            ImageFormat::Avif => write!(f, "image/avif"),
            ImageFormat::Jpeg => write!(f, "image/jpeg"),
        }
    }
}

impl ImageFormat {
    fn extension(self) -> &'static str {
        match self {
            ImageFormat::WebP => "webp",
            ImageFormat::Avif => "avif",
            ImageFormat::Jpeg => "jpg",
        }
    }

    fn target_format(self) -> image::ImageFormat {
        match self {
            ImageFormat::WebP => image::ImageFormat::WebP,
            ImageFormat::Avif => image::ImageFormat::Avif,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
        }
    }
}

pub struct Picture<'a, Context> {
    handle: &'a Handle,
    class: Option<String>,
    alt: Cow<'a, str>,
    context: PhantomData<Context>,
}

impl<'a, Context> Picture<'a, Context> {
    pub fn new(handle: &'a Handle) -> Self {
        Self {
            handle,
            class: None,
            alt: Cow::Borrowed(""),
            context: PhantomData,
        }
    }

    pub fn class(mut self, class: impl Class<Context>) -> Self {
        self.class = Some(class.resolve());
        self
    }

    pub fn alt(mut self, alt: impl Into<Cow<'a, str>>) -> Self {
        self.alt = alt.into();
        self
    }
}

impl<'a, Context> Widget<Context> for Picture<'a, Context> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<picture")?;
        if let Some(class) = &self.class {
            write!(f, " class=\"{}\"", class)?;
        }
        write!(f, ">")?;

        let mut grouped: BTreeMap<ImageFormat, Vec<&PictureVersion>> = BTreeMap::new();
        for version in &self.handle.versions {
            grouped.entry(version.format).or_default().push(version);
        }

        for format in [ImageFormat::Avif, ImageFormat::WebP, ImageFormat::Jpeg] {
            let Some(versions) = grouped.get(&format) else {
                continue;
            };
            let srcset = versions
                .iter()
                .map(|version| format!("{} {}w", version.location, version.resolution.0))
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "<source srcset=\"{}\" type=\"{}\">", srcset, format)?;
        }

        write!(
            f,
            "<img src=\"{}\" width=\"{}\" height=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\">",
            self.handle.fallback.location,
            self.handle.fallback.resolution.0,
            self.handle.fallback.resolution.1,
            crate::html_sanitize(&self.alt)
        )?;
        write!(f, "</picture>")?;

        Ok(())
    }

    fn style(&self, _theme: &dyn crate::theme::Theme, _stylesheet: &mut crate::style::Stylesheet) {}
}

static BUILD_OUTPUT_DIR: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();
static BUILD_CACHE_DIR: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

fn build_output_dir() -> &'static Mutex<Option<PathBuf>> {
    BUILD_OUTPUT_DIR.get_or_init(|| Mutex::new(None))
}

fn build_cache_dir() -> &'static Mutex<Option<PathBuf>> {
    BUILD_CACHE_DIR.get_or_init(|| Mutex::new(None))
}

pub struct BuildContext;

impl BuildContext {
    pub(crate) fn new(output_dir: &Path) -> Self {
        let output_dir = output_dir.to_path_buf();
        let cache_dir = output_dir
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(".cache")
            .join("pictures");

        let mut output_guard = build_output_dir()
            .lock()
            .expect("picture build context mutex poisoned");
        *output_guard = Some(output_dir);

        let mut cache_guard = build_cache_dir()
            .lock()
            .expect("picture build context mutex poisoned");
        *cache_guard = Some(cache_dir);

        Self
    }
}

impl Drop for BuildContext {
    fn drop(&mut self) {
        let mut output_guard = build_output_dir()
            .lock()
            .expect("picture build context mutex poisoned");
        *output_guard = None;

        let mut cache_guard = build_cache_dir()
            .lock()
            .expect("picture build context mutex poisoned");
        *cache_guard = None;
    }
}

struct SourceImage {
    path: PathBuf,
    bytes: Vec<u8>,
    image: DynamicImage,
}

impl SourceImage {
    fn load(path: &Path) -> Result<Self, anyhow::Error> {
        let bytes =
            fs::read(path).with_context(|| format!("Failed to read picture {}", path.display()))?;
        let image = ImageReader::new(Cursor::new(bytes.as_slice()))
            .with_guessed_format()
            .with_context(|| format!("Failed to detect picture format {}", path.display()))?
            .decode()
            .with_context(|| format!("Failed to decode picture {}", path.display()))?;

        Ok(Self {
            path: path.to_path_buf(),
            bytes,
            image,
        })
    }

    fn checksum(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.bytes.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.image.width(), self.image.height())
    }
}

fn build_variants(source: &Path, source_image: &SourceImage) -> Vec<PictureVersion> {
    let (width, height) = source_image.dimensions();
    let widths = variant_widths(width);
    let slug = source_image.checksum();
    let mut versions = Vec::new();

    for format in [ImageFormat::Avif, ImageFormat::WebP, ImageFormat::Jpeg] {
        for target_width in &widths {
            let target_height = scaled_height(width, height, *target_width);
            versions.push(PictureVersion {
                resolution: (*target_width, target_height),
                format,
                location: format!(
                    "/assets/pictures/{}/{}-{}w.{}",
                    slug,
                    file_stem(source),
                    target_width,
                    format.extension()
                ),
            });
        }
    }

    versions
}

fn generate_variants(
    source_image: &SourceImage,
    versions: &[PictureVersion],
) -> Result<(), anyhow::Error> {
    let output_dir = {
        let guard = build_output_dir()
            .lock()
            .expect("picture build context mutex poisoned");
        guard
            .clone()
            .context("PictureHandle::create() must be called during a site build")?
    };
    let cache_dir = {
        let guard = build_cache_dir()
            .lock()
            .expect("picture build context mutex poisoned");
        guard
            .clone()
            .context("PictureHandle::create() must be called during a site build")?
    };

    println!("Building picture: {}", source_image.path.display());

    let cache_folder = cache_dir.join(source_image.checksum());
    fs::create_dir_all(&cache_folder).with_context(|| {
        format!(
            "Failed to create picture cache directory {}",
            cache_folder.display()
        )
    })?;

    versions.par_iter().try_for_each(|version| {
        ensure_picture_version(source_image, version, &cache_folder, &output_dir)
    })
}

fn ensure_picture_version(
    source_image: &SourceImage,
    version: &PictureVersion,
    cache_folder: &Path,
    output_dir: &Path,
) -> Result<(), anyhow::Error> {
    let cached = cache_path(cache_folder, source_image.path.as_path(), version);
    if !cached.exists() {
        let resized = resize_to_width(
            &source_image.image,
            version.resolution.0,
            version.resolution.1,
        );
        write_variant(&resized, version.format, &cached)?;
    }

    let destination = output_path(output_dir, version);
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create picture output directory {}",
                parent.display()
            )
        })?;
    }

    fs::copy(&cached, &destination).with_context(|| {
        format!(
            "Failed to copy picture variant from {} to {}",
            cached.display(),
            destination.display()
        )
    })?;

    Ok(())
}

fn variant_widths(original_width: u32) -> Vec<u32> {
    let mut widths = vec![320, 640, 960, 1280, 1600, original_width];
    widths.retain(|width| *width <= original_width);
    widths.sort_unstable();
    widths.dedup();
    widths
}

fn scaled_height(original_width: u32, original_height: u32, target_width: u32) -> u32 {
    ((u64::from(original_height) * u64::from(target_width)) / u64::from(original_width))
        .try_into()
        .unwrap_or(original_height)
}

fn cache_path(cache_folder: &Path, source: &Path, version: &PictureVersion) -> PathBuf {
    cache_folder.join(format!(
        "{}-{}w.{}",
        file_stem(source),
        version.resolution.0,
        version.format.extension()
    ))
}

fn output_path(output_dir: &Path, version: &PictureVersion) -> PathBuf {
    output_dir.join(version.location.trim_start_matches('/'))
}

fn file_stem(source: &Path) -> String {
    source
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(sanitize_segment)
        .filter(|stem| !stem.is_empty())
        .unwrap_or_else(|| "picture".to_string())
}

fn sanitize_segment(segment: &str) -> String {
    segment
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

fn resize_to_width(original: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    if original.width() == width && original.height() == height {
        return original.clone();
    }
    original.resize_exact(width, height, FilterType::Lanczos3)
}

fn write_variant(
    image: &DynamicImage,
    format: ImageFormat,
    destination: &Path,
) -> Result<(), anyhow::Error> {
    let mut file = File::create(destination)
        .with_context(|| format!("Failed to create {}", destination.display()))?;

    match format {
        ImageFormat::Jpeg => flatten_for_jpeg(image).write_to(&mut file, format.target_format()),
        _ => image.write_to(&mut file, format.target_format()),
    }
    .with_context(|| format!("Failed to encode {}", destination.display()))
}

fn flatten_for_jpeg(image: &DynamicImage) -> DynamicImage {
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    let mut rgb = image::RgbImage::new(width, height);

    for (x, y, pixel) in rgba.enumerate_pixels() {
        let alpha = u16::from(pixel[3]);
        let red = blend_channel(pixel[0], alpha);
        let green = blend_channel(pixel[1], alpha);
        let blue = blend_channel(pixel[2], alpha);
        rgb.put_pixel(x, y, image::Rgb([red, green, blue]));
    }

    DynamicImage::ImageRgb8(rgb)
}

fn blend_channel(channel: u8, alpha: u16) -> u8 {
    let foreground = u16::from(channel) * alpha;
    let background = 255u16 * (255 - alpha);
    ((foreground + background) / 255) as u8
}
