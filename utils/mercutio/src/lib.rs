//! A "CSS in Rust" crate. Mercutio is pretty simple.
//!
//! At its heart is a trait called `Apply<T>` which has a function
//! `fn apply(self, to_apply: T) -> Self` that applies `T` to `self` and returns the modified
//! `Self`.
//!
//! But what is `T` and how does one get an instance of T? Use the `CSS` macro. The `CSS` macro is
//! a derive macro which automatically implements `Apply<ITEM>` for the `ITEM` on which it is
//! derived. For example:
//!
//! ```rust
//! # use mercutio::*;
//! #[derive(CSS)]
//! #[mercutio(
//!     css(font_family = "sans-serif", font_size = "24px"),
//!     elements(H1, H2, H3)
//! )]
//! struct Title;
//! ```
//!
//! You can then apply `Title` to any of the `H1`, `H2` or `H3` tags – for example:
//!
//! ```rust
//! # use mercutio::*;
//! # use malvolio::prelude::H1;
//! # #[derive(CSS)]
//! # #[mercutio(
//! #     css(font_family = "sans-serif", font_size = "24px"),
//! #     elements(H1, H2, H3)
//! # )]
//! # struct Title;
//! H1::new("Title").apply(Title);
//! ```

#![deny(missing_debug_implementations, missing_docs)]

pub use mercutio_codegen::CSS;

/// A trait which applies the relevant CSS styles to an item.
///
/// You can (but probably don't want to) implement this yourself, but we suggest using the derive
/// macro we provide.
///
/// ```rust
/// # use mercutio::*;
/// #[derive(CSS)]
/// #[mercutio(
///     css(font_family = "sans-serif", font_size = "24px"),
///     elements(H1, H2, H3)
/// )]
/// struct Title;
/// ```
pub trait Apply<T> {
    /// Apply the provided item to the current function. In many cases the item is not actually
    /// used, however, there are many cases where this is the case.
    fn apply(self, to_apply: T) -> Self;
}

/// Allows you to compose different CSS styles together. This function takes
///
/// ```rust
/// # use mercutio::*;
/// # use malvolio::prelude::H1;
///
/// #[derive(CSS)]
/// #[mercutio(
///     css(color = "blue"),
///     elements(H1)
/// )]
/// struct Blue;
///
/// #[derive(CSS)]
/// #[mercutio(
///     elements(H1),
///     css(font_size="24px")
/// )]
/// struct Title;
///
/// let blue_title = compose(Blue, Title);
/// H1::new("Blue title").apply(blue_title);
/// ```
///
/// ```rust
/// # use mercutio::*;
/// # use malvolio::prelude::H1;
///
/// #[derive(CSS)]
/// #[mercutio(
///     css(color = "blue"),
///     elements(H1)
/// )]
/// struct Blue;
///
/// #[derive(CSS)]
/// #[mercutio(
///     elements(H1),
///     css(font_size="24px")
/// )]
/// struct Title;
///
/// #[derive(CSS)]
/// #[mercutio(
///     css(background_color = "blue"),
///     elements(H1)
/// )]
/// struct BlueBackground;
///
/// let composed = compose(BlueBackground, compose(Blue, Title));
/// H1::new("Blue title with blue background.").apply(composed);
/// ```
pub fn compose<T, A, B>(a: A, b: B) -> impl FnOnce(T) -> T
where
    T: Apply<A> + Apply<B>,
{
    move |t: T| t.apply(a).apply(b)
}

impl<F, T> Apply<F> for T
where
    F: FnOnce(T) -> T,
{
    fn apply(self, to_apply: F) -> Self {
        to_apply(self)
    }
}

/// This trait contains a constant called "CLASS" which allows access to the class name for this
/// item. The `#[derive(CSS)]` macro will output this, if you enable the "classes=true" option.
/// Note that if you do not enable that option, this trait will not be derived and you will get a
/// compile error.
pub trait ClassName {
    /// The class name corresponding.
    const CLASS: &'static str;
}
