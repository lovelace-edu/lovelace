use std::borrow::Cow;

use malvolio::{prelude::*, text::Text};
use mercutio::{compose, Apply};

use crate::{levels::Level, padding::DefaultPadding, render::Render};
#[derive(Derivative, Debug, Clone)]
#[derivative(Default(new = "true"))]
pub struct Card {
    title: Cow<'static, str>,
    contents: Option<CardContents>,
    action_bar: ActionBar,
}

impl Card {
    pub fn title<C>(mut self, c: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        self.title = c.into();
        self
    }
    pub fn contents<C>(mut self, c: C) -> Self
    where
        C: Into<CardContents>,
    {
        self.contents = Some(c.into());
        self
    }
    pub fn action_bar<I, A>(mut self, actions: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Action>,
    {
        self.action_bar
            .items
            .extend(actions.into_iter().map(Into::into));
        self
    }
}

#[derive(mercutio::CSS)]
#[mercutio(elements(Div), css(border = "1px solid black"))]
pub struct CardStyle;

#[derive(mercutio::CSS)]
#[mercutio(elements(Div), css(border_bottom = "1px solid black"))]
pub struct CardTitleSlide;

impl Render<Div> for Card {
    fn render(self) -> Div {
        let (contents, action_bar) = (self.contents, self.action_bar);
        Div::new()
            .apply(CardStyle)
            .child(
                Div::new()
                    .apply(compose(CardTitleSlide, DefaultPadding))
                    .child(P::with_text(self.title.clone())),
            )
            .map(|div| {
                if let Some(contents) = contents {
                    div.child(match contents {
                        CardContents::Text(contents) => BodyNode::from(Text::new(contents)),
                        CardContents::Other(node) => node,
                    })
                } else {
                    div
                }
            })
            .child(
                Level::new().children(action_bar.items.into_iter().map(|action| match action {
                    Action::Link(link) => link.into(),
                    Action::Other(node) => node,
                })),
            )
    }
}

#[derive(Debug, Clone)]
pub enum CardContents {
    Text(Cow<'static, str>),
    Other(BodyNode),
}

impl From<Cow<'static, str>> for CardContents {
    fn from(text: Cow<'static, str>) -> Self {
        CardContents::Text(text)
    }
}

impl From<BodyNode> for CardContents {
    fn from(node: BodyNode) -> Self {
        CardContents::Other(node)
    }
}

#[derive(Derivative, Debug, Clone)]
#[derivative(Default(new = "true"))]
pub struct ActionBar {
    items: Vec<Action>,
}

impl ActionBar {
    /// Add a child to this action bar. You *can* provide actions other than links to this function,
    /// but we don't recommend that.
    pub fn child<C>(mut self, child: C) -> Self
    where
        C: Into<Action>,
    {
        self.items.push(child.into());
        self
    }

    /// Attach a number of children to this item.
    pub fn children<I, C>(mut self, iterator: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Action>,
    {
        self.items.extend(iterator.into_iter().map(Into::into));
        self
    }
}

#[derive(Debug, Clone)]
/// An action.
pub enum Action {
    Link(A),
    Other(BodyNode),
}

impl From<A> for Action {
    fn from(a: A) -> Self {
        Self::Link(a)
    }
}

impl From<BodyNode> for Action {
    fn from(b: BodyNode) -> Self {
        Self::Other(b)
    }
}
