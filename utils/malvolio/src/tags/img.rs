#[cfg(feature = "with_yew")]
use std::rc::Rc;
use std::{borrow::Cow, collections::HashMap, fmt::Display};
#[cfg(feature = "with_yew")]
use yew::virtual_dom::Listener;

use crate::{
    attributes::IntoAttribute, into_attribute_for_grouping_enum, into_grouping_union, utility_enum,
};

#[derive(Debug, Derivative)]
#[derivative(Default(new = "true"))]
/// The `<img>` tag.
pub struct Img {
    attrs: HashMap<&'static str, Cow<'static, str>>,
    #[cfg(feature = "with_yew")]
    #[cfg(not(tarpaulin))]
    listeners: Vec<Rc<dyn Listener>>,
}

impl Display for Img {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<img")?;
        for attr in &self.attrs {
            f.write_str(" ")?;
            attr.0.fmt(f)?;
            f.write_str("=\"")?;
            attr.1.fmt(f)?;
            f.write_str("\"")?;
        }
        f.write_str("\"")?;
        f.write_str("/>")
    }
}

#[cfg(feature = "with_yew")]
#[cfg(not(tarpaulin))]
impl crate::into_vnode::IntoVNode for Img {
    fn into_vnode(self) -> yew::virtual_dom::VNode {
        let mut vnode = yew::virtual_dom::VTag::new("img");
        for (a, b) in self.attrs {
            vnode.add_attribute(a, &b.to_string())
        }
        vnode.into()
    }
}

impl Img {
    /// Attach an attribute to the <img> tag in question.
    pub fn attribute<A>(mut self, attribute: A) -> Self
    where
        A: Into<ImgAttr>,
    {
        let res = attribute.into().into_attribute();
        self.attrs.insert(res.0, res.1);
        self
    }

    /// Read an attribute that has been set.
    pub fn read_attribute(&self, attribute: &'static str) -> Option<&Cow<'static, str>> {
        self.attrs.get(attribute)
    }
}

utility_enum!(
    pub enum ImgAttr {
        /// The `alt` attribute.
        Alt(Alt),
        /// The `src` attribute.
        Src(Src),
    }
);

into_attribute_for_grouping_enum!(ImgAttr, Alt, Src);

#[derive(Debug, Clone)]
/// The `alt` attribute.
pub struct Alt {
    value: Cow<'static, str>,
}

into_grouping_union!(Alt, ImgAttr);

impl Alt {
    /// Construct a new instance of this attribute.
    pub fn new<C>(c: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self { value: c.into() }
    }
}

impl IntoAttribute for Alt {
    fn into_attribute(self) -> (&'static str, Cow<'static, str>) {
        ("alt", self.value)
    }
}

#[derive(Debug, Clone)]
/// The `src` attribute.
pub struct Src {
    src: Cow<'static, str>,
}

into_grouping_union!(Src, ImgAttr);

impl Src {
    /// Construct a new instance of this attribute.
    pub fn new<C>(c: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self { src: c.into() }
    }
}

impl IntoAttribute for Src {
    fn into_attribute(self) -> (&'static str, Cow<'static, str>) {
        ("src", self.src)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_img() {
        let document = Img::new()
            .attribute(Src::new("cat.jpeg"))
            .attribute(Alt::new(
                "An animated picture of a cat doing some humorous task.",
            ))
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let a = scraper::Selector::parse("img").unwrap();
        let a = document.select(&a).next().unwrap().value();
        assert_eq!(a.attr("src").unwrap(), "cat.jpeg");
        assert_eq!(
            a.attr("alt").unwrap(),
            "An animated picture of a cat doing some humorous task."
        );
    }
}
