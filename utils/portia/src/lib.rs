//! Portia is Lovelace's UI library.

#[macro_use]
extern crate derivative;
#[macro_use]
extern crate mercutio;

/// A "card" which is useful for displaying information.
pub mod card;
pub mod colour;
/// Font primitives (ok, not *that* primitive, but at the level of abstraction at which this
/// library works they're relatively speaking quite primitive).
pub mod font;
/// Form stylings, which we use in Lovelace. These may or may not work for other projects depending
/// on whether they match the look and feel of your implementations.
pub mod form;
/// A flexbox wrapper for creating layouts.
pub mod levels;
/// Margin primitives (ok, not *that* primitive, but at the level of abstraction at which this
/// library works they're relatively speaking quite primitive).
pub mod margin;
/// Padding primitives (ok, not *that* primitive, but at the level of abstraction at which this
/// library works they're relatively speaking quite primitive).
pub mod padding;
/// Contains the `Render` trait, which provides a method to render anything into an "X". This is
/// functionally very similar to `From`, but it does make things less ambigous in many cases.
pub mod render;
