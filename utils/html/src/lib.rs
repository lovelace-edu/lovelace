//! A small set of data types for producing HTML code.

use std::{fmt::Display, io::Cursor};

use rocket::{response::Responder, Response};

macro_rules! heading_display {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("<")?;
                f.write_str(stringify!($name))?;
                f.write_str(">")?;
                self.0.fmt(f)?;
                f.write_str("</")?;
                f.write_str(stringify!($name))?;
                f.write_str(">")
            }
        }
    };
}

#[derive(Default, Clone, Debug)]
pub struct Html {
    head: Head,
    body: Body,
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

impl<'r> Responder<'r> for Html {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'r> {
        Response::build()
            .raw_header("Content-Type", "text/html")
            .sized_body(Cursor::new(self.to_string()))
            .ok()
    }
}

impl Html {
    pub fn head(mut self, head: Head) -> Self {
        self.head = head;
        self
    }
    pub fn body(mut self, body: Body) -> Self {
        self.body = body;
        self
    }
}

#[derive(Default, Debug, Clone)]
pub struct Head {
    children: Vec<HeadNode>,
}

impl Head {
    pub fn children(mut self, children: Vec<HeadNode>) -> Self {
        self.children.extend(children);
        self
    }
    pub fn child(mut self, child: HeadNode) -> Self {
        self.children.push(child);
        self
    }
}

impl Display for Head {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<head>")?;
        for child in &self.children {
            child.fmt(f)?;
        }
        f.write_str("</head>")
    }
}

#[derive(Debug, Clone)]
pub enum HeadNode {
    Title(Title),
    Meta(Meta),
}

#[derive(Default, Debug, Clone)]
pub struct Meta {
    attrs: Vec<(String, String)>,
}

impl Meta {
    pub fn attribute(mut self, k: String, v: String) -> Self {
        self.attrs.push((k, v));
        self
    }
}

impl Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<meta")?;
        for attr in &self.attrs {
            f.write_str(" ")?;
            attr.0.fmt(f)?;
            f.write_str("=\"")?;
            attr.1.fmt(f)?;
            f.write_str("\"")?;
        }
        f.write_str("/>")
    }
}

#[derive(Debug, Clone)]
pub struct Stylesheet {
    href: String,
}

impl Display for Stylesheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<link rel=\"stylesheet\" href=\"")?;
        f.write_str(&self.href)?;
        f.write_str("\"/>")
    }
}

impl Stylesheet {
    pub fn new(href: String) -> Self {
        Self { href }
    }
}

#[derive(Debug, Clone)]
pub struct Title(pub String);

heading_display!(Title);

#[derive(Default, Debug, Clone)]
pub struct Body {
    children: Vec<BodyNode>,
}

impl Body {
    pub fn children(mut self, children: Vec<BodyNode>) -> Self {
        self.children.extend(children);
        self
    }
    pub fn child(mut self, child: BodyNode) -> Self {
        self.children.push(child);
        self
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<body>")?;
        for node in &self.children {
            node.fmt(f)?;
        }
        f.write_str("</body>")
    }
}

macro_rules! enum_display {
    ($on:ident, $($variant:ident),*) => {
        impl std::fmt::Display for $on {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(x) => <$variant as std::fmt::Display>::fmt(x, f)),*
                }
            }
        }
    };
}

enum_display!(HeadNode, Title, Meta);

#[derive(Clone, Debug)]
pub enum BodyNode {
    H1(H1),
    H2(H2),
    H3(H3),
    H4(H4),
    H5(H5),
    H6(H6),
    P(P),
    Text(Text),
}

macro_rules! into_grouping_union {
    ($name:ident, $enum_name:ident) => {
        impl From<$name> for $enum_name {
            fn from(t: $name) -> $enum_name {
                $enum_name::$name(t)
            }
        }
    };
}

into_grouping_union!(Meta, HeadNode);
into_grouping_union!(Title, HeadNode);

enum_display!(BodyNode, H1, H2, H3, H4, H5, H6, P, Text);

#[derive(Default, Debug, Clone)]
pub struct H1(pub String);

into_grouping_union!(H1, BodyNode);

heading_display!(H1);

#[derive(Default, Debug, Clone)]
pub struct H2(pub String);

into_grouping_union!(H2, BodyNode);

heading_display!(H2);

#[derive(Default, Debug, Clone)]
pub struct H3(pub String);

into_grouping_union!(H3, BodyNode);

heading_display!(H3);

#[derive(Default, Debug, Clone)]
pub struct H4(pub String);

into_grouping_union!(H4, BodyNode);

heading_display!(H4);

#[derive(Default, Debug, Clone)]
pub struct H5(pub String);

into_grouping_union!(H5, BodyNode);

heading_display!(H5);

#[derive(Default, Debug, Clone)]
pub struct H6(pub String);

into_grouping_union!(H6, BodyNode);

heading_display!(H6);

#[derive(Default, Debug, Clone)]
pub struct P {
    children: Vec<BodyNode>,
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
    pub fn children(mut self, children: Vec<BodyNode>) -> Self {
        self.children.extend(children);
        self
    }
    pub fn child(mut self, child: BodyNode) -> Self {
        self.children.push(child);
        self
    }
    pub fn text(self, text: String) -> Self {
        self.child(BodyNode::Text(Text(text)))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Text(pub String);

into_grouping_union!(Text, BodyNode);

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
