/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, fmt::Display};

use super::body::body_node::BodyNode;
#[cfg(feature = "with_yew")]
#[cfg(not(tarpaulin))]
use crate::into_vnode::IntoVNode;
use crate::{into_grouping_union, text::Text};
use ammonia::clean;

/// The <p> tag.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p) for more
/// info.
#[derive(Default, Debug, Clone)]
pub struct P {
    children: Vec<BodyNode>,
}

#[cfg(feature = "with_yew")]
#[cfg(not(tarpaulin))]
impl IntoVNode for P {
    fn into_vnode(self) -> yew::virtual_dom::VNode {
        let mut vtag = yew::virtual_dom::VTag::new("br");
        vtag.add_children(self.children.into_iter().map(IntoVNode::into_vnode));
        vtag.into()
    }
}

into_grouping_union!(P, BodyNode);

impl Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<p>")?;
        for child in &self.children {
            child.fmt(f)?;
        }
        f.write_str("</p>")
    }
}

impl P {
    /// Adds multiple children to the current `P` node after the currently existing ones. This
    /// method accepts any
    pub fn children<C>(mut self, children: Vec<C>) -> Self
    where
        C: Into<BodyNode>,
    {
        self.children
            .extend(children.into_iter().map(Into::into).collect::<Vec<_>>());
        self
    }
    /// Attach a child to this tag.
    pub fn child<C>(mut self, child: C) -> Self
    where
        C: Into<BodyNode>,
    {
        self.children.push(child.into());
        self
    }
    /// A method to construct a paragraph containing the supplied text. This will sanitise the text
    /// provided beforehand.
    pub fn with_text<S>(text: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        P::default().child(BodyNode::Text(Text::new(clean(&text.into()))))
    }
    /// Adds the supplied text to this node, overwriting the previously existing text (if text has
    /// already been added to the node).
    ///
    /// This method sanitises the input (i.e. it escapes HTML);
    /// this might not be what you want – if you are *absolutely certain* that the text you are
    /// providing does not come from a potentially malicious source (e.g. user-supplied text can
    /// contain script tags which will execute unwanted code) you can use `text_unsanitized` which
    /// is identical to this method, except for that it does not sanitise the inputted text (and is
    /// thus slightly faster).
    pub fn text<S>(self, text: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.child(BodyNode::Text(Text::new(clean(&text.into()))))
    }
    /// Adds the supplied text to this node, overwriting the previously existing text (if text has
    /// already been added to the node).
    ///
    /// WARNING: Do not (under any circumstances) use this method with unescaped user-supplied text.
    /// It will be rendered and poses a major security threat to your application. If in doubt, use
    /// the `text` method instead of this one (the risk is much lower that way).
    pub fn text_unsanitized<S>(self, text: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.child(BodyNode::Text(Text::new(text)))
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_p() {
        let document = P::with_text("Some text").to_string();
        let document = scraper::Html::parse_document(&document);
        let p = scraper::Selector::parse("p").unwrap();
        let p = document.select(&p).next().unwrap();
        assert_eq!(
            p.children()
                .next()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .to_string()
                .as_str(),
            "Some text"
        );
    }
}
