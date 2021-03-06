/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::fmt::Display;

#[cfg(feature = "with_rocket")]
use std::io::Cursor;

#[cfg(feature = "with_rocket")]
use rocket::{response::Responder, Response};

#[cfg(feature = "with_yew")]
#[cfg(not(tarpaulin))]
use crate::into_vnode::IntoVNode;
use crate::to_html;

use super::{body::Body, head::Head};

#[derive(Clone, Debug)]
/// Construct a HTML document. If you are trying to render to a string, this is what you want to use.
///
/// If you're using Yew (enable the `with_yew` feature in your `Cargo.toml` to do this) then you
/// probably want to use the relevant tag which your component should return instead.
pub struct Html {
    #[cfg(feature = "with_rocket")]
    status: u16,
    #[cfg(feature = "with_rocket")]
    reason: Option<&'static str>,
    head: Head,
    body: Body,
}

#[cfg(feature = "with_yew")]
#[cfg(not(tarpaulin))]
impl IntoVNode for Html {
    fn into_vnode(self) -> yew::virtual_dom::VNode {
        let mut tag = yew::virtual_dom::VTag::new("html");
        tag.add_children(vec![
            IntoVNode::into_vnode(self.head),
            IntoVNode::into_vnode(self.body),
        ]);
        tag.into()
    }
}

impl Default for Html {
    fn default() -> Self {
        Self {
            #[cfg(feature = "with_rocket")]
            status: 200,
            #[cfg(feature = "with_rocket")]
            reason: None,
            head: Head::default(),
            body: Body::default(),
        }
    }
}

impl Display for Html {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<!DOCTYPE html>")?;
        f.write_str("<html>")?;
        self.head.fmt(f)?;
        self.body.fmt(f)?;
        f.write_str("</html>")?;
        Ok(())
    }
}

#[cfg(feature = "with_rocket")]
impl<'r, 'o: 'r> Responder<'r, 'o> for Html {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'o> {
        Response::build()
            .raw_status(self.status, self.reason.unwrap_or(""))
            .raw_header("Content-Type", "text/html")
            .streamed_body(Cursor::new(self.to_string()))
            .ok()
    }
}

impl Html {
    /// Create a new `Html` tag. Note that this is exactly the same as `Html::default()`, but it is
    /// a few characters shorter, so is provided as a convenience method.
    pub fn new() -> Self {
        Self::default()
    }
    /// Attach a <head> tag to this `Html` instance.
    pub fn head(mut self, head: Head) -> Self {
        self.head = head;
        self
    }
    /// Attach a new <body> tag to this `Html` instance.
    pub fn body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }
    #[cfg(feature = "with_rocket")]
    /// Add the corresponding status code to return this HTML document with. Note that this is only
    /// available if you have enabled the `with_rocket` feature.
    pub fn status(mut self, code: u16) -> Self {
        self.status = code;
        self
    }
    #[cfg(feature = "with_rocket")]
    /// Add a reason for to return alongside the status of this HTML document with. Note that this
    /// is only available if you have enabled the `with_rocket` feature.
    pub fn status_reason(mut self, reason: &'static str) -> Self {
        self.reason = Some(reason);
        self
    }
    to_html!();
}
