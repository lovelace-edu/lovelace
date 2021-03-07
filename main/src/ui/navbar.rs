use malvolio::{prelude::*, text::Text};
use mercutio::{compose, Apply};
use portia::{
    colour::YellowBackground,
    font::{SmallTitle, VerticalAlignCenter},
    levels::{LayoutAxis, LayoutStrategy, Level, Spacing},
    padding::DefaultPadding,
    render::Render,
};

pub struct Navbar;

impl Render<Div> for Navbar {
    fn render(self) -> Div {
        Level::new()
            .strategy(
                LayoutStrategy::new()
                    .axis(LayoutAxis::Horizontal)
                    .spacing(Spacing::Between),
            )
            .child(H1::new("Lovelace").apply(SmallTitle))
            .child(
                Div::new()
                    .apply(VerticalAlignCenter)
                    .attribute(Id::new("auth-bar"))
                    .child(A::new().href("/auth/login").text("Login"))
                    .child(Text::new(" "))
                    .child(A::new().href("/auth/register").text("Register")),
            )
            .into_div()
            .apply(compose(YellowBackground, DefaultPadding))
    }
}
