pub mod builder;
pub mod layout;
pub mod page;
pub mod style;
pub mod theme;
pub mod widget;
use std::{any::TypeId, marker::PhantomData};

pub use widget::helpers::*;

pub fn html_sanitize<'a>(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    let mut sanitized = String::with_capacity(input.len());

    for char in input.chars() {
        let replacement = match char {
            '&' => "&amp;",
            '>' => "&gt;",
            '<' => "&lt;",
            '"' => "&quot;",
            '\'' => "&#x27;",
            '\\' => "&#x2F;",
            char => {
                sanitized.push(char);
                continue;
            }
        };

        sanitized.push_str(replacement);
    }

    sanitized
}

#[must_use]
#[inline(always)]
pub fn type_id<T>() -> TypeId
where
    T: ?Sized,
{
    trait NonStaticAny {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static;
    }

    impl<T: ?Sized> NonStaticAny for PhantomData<T> {
        #[inline(always)]
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static,
        {
            TypeId::of::<T>()
        }
    }

    let phantom_data = PhantomData::<T>;
    NonStaticAny::get_type_id(unsafe {
        std::mem::transmute::<&dyn NonStaticAny, &(dyn NonStaticAny + 'static)>(&phantom_data)
    })
}
