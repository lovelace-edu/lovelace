use std::fmt::Display;

#[cfg(feature = "with_yew")]
use crate::into_vnode::IntoVNode;
use crate::{add_single_attribute, into_grouping_union, to_html};

use super::body::body_node::BodyNode;

#[derive(Debug, Clone, Default)]
pub struct Form {
    children: Vec<BodyNode>,
    attrs: Vec<(&'static str, &'static str)>,
}

#[cfg(feature = "with_yew")]
impl IntoVNode for Form {
    fn into(self) -> yew::virtual_dom::VNode {
        let mut vtag = yew::virtual_dom::VTag::new("form");
        vtag.add_children(self.children.into_iter().map(IntoVNode::into));
        for (a, b) in self.attrs {
            vtag.add_attribute(a, &b.to_string())
        }
        vtag.into()
    }
}

impl Form {
    #[inline(always)]
    pub fn children<C>(mut self, children: Vec<C>) -> Self
    where
        C: Into<BodyNode>,
    {
        self.children
            .extend(children.into_iter().map(Into::into).collect::<Vec<_>>());
        self
    }
    #[inline(always)]
    pub fn child<C>(mut self, child: C) -> Self
    where
        C: Into<BodyNode>,
    {
        self.children.push(child.into());
        self
    }
    add_single_attribute!('static);
    to_html!();
}

impl Display for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<form ")?;
        for attr in &self.attrs {
            f.write_str(" ")?;
            attr.0.fmt(f)?;
            f.write_str("=\"")?;
            attr.1.fmt(f)?;
            f.write_str("\"")?;
        }
        f.write_str(">")?;
        for node in &self.children {
            node.fmt(f)?;
        }
        f.write_str("</form>")
    }
}

into_grouping_union!(Form, BodyNode);

#[cfg(test)]
mod form {
    use crate::prelude::*;
    #[test]
    fn test_form_tag() {
        let document = Form::default()
            .attribute("method", "post")
            .attribute("action", "/")
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let form = scraper::Selector::parse("form").unwrap();
        let form = document.select(&form).next().unwrap().value();
        assert_eq!(form.attr("method"), Some("post"));
        assert_eq!(form.attr("action"), Some("/"));
    }
    #[test]
    fn test_form_with_children() {
        let document = Form::default()
            .child(
                Input::default()
                    .attribute("type", "text")
                    .attribute("name", "input1"),
            )
            .child(Input::default().attribute("type", "submit"))
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let input = scraper::Selector::parse("input").unwrap();
        let inputs = document.select(&input).collect::<Vec<_>>();
        assert_eq!(inputs.len(), 2);
        let input1 = inputs[0].value();
        assert_eq!(input1.attr("type"), Some("text"));
        assert_eq!(input1.attr("name"), Some("input1"));
        let input2 = inputs[1].value();
        assert_eq!(input2.attr("type"), Some("submit"))
    }
}