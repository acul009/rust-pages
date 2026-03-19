use crate::{style::Class, widget::Widget};
use std::fmt::{Display, Write};

pub struct PictureHandle {
    versions: Vec<PictureVersion>,
}

impl PictureHandle {
    pub fn create(original: &str) -> Result<Self, anyhow::Error> {
        todo!()
    }
}

struct PictureVersion {
    resolution: (u32, u32),
    format: ImageFormat,
    location: String,
}

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

pub struct Picture<'a> {
    handle: &'a PictureHandle,
    class: Option<String>,
}

impl<'a> Picture<'a> {
    pub fn new(handle: &'a PictureHandle) -> Self {
        Self {
            handle,
            class: None,
        }
    }

    pub fn class<Context>(mut self, class: impl Class<Context>) -> Self {
        self.class = Some(class.resolve());
        self
    }
}

impl<'a, Context> Widget<Context> for Picture<'a> {
    fn html(&self, f: &mut String) -> std::fmt::Result {
        write!(f, "<picture")?;
        if let Some(class) = &self.class {
            write!(f, " class=\"{}\"", class)?;
        }
        write!(f, ">")?;
        for version in &self.handle.versions {
            write!(
                f,
                "<source srcset=\"{}\" type=\"image/{}\">",
                version.location, version.format
            )?;
        }
        write!(f, "</picture>")?;

        Ok(())
    }

    fn style(&self, theme: &dyn crate::theme::Theme, stylesheet: &mut crate::style::Stylesheet) {}
}
