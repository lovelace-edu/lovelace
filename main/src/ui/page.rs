use malvolio::prelude::*;
use mercutio::Apply;
use portia::{margin::ZeroMargin, render::Render};

use super::navbar::Navbar;

#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct Page {
    children: Vec<BodyNode>,
}

impl Page {
    pub fn child<C>(mut self, child: C) -> Self
    where
        C: Into<BodyNode>,
    {
        self.children.push(child.into());
        self
    }
    #[allow(unused)]
    pub fn children<C, I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<BodyNode>,
    {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }
}

impl Render<Body> for Page {
    fn render(self) -> Body {
        Body::new()
            .apply(ZeroMargin)
            .child(Render::<Div>::render(Navbar))
            .children(self.children)
    }
}
