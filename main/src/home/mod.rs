use malvolio::{prelude::*, text::Text};
use mercutio::{compose, Apply};
use portia::{
    colour::{GreyBackground, YellowBackground},
    font::{SmallTitle, VerticalAlignCenter},
    levels::{LayoutAxis, LayoutStrategy, Level, Spacing},
    margin::ZeroMargin,
    padding::DefaultPadding,
};
use rocket::response::Redirect;

use crate::{auth::OptionAuthCookie, utils::html_or_redirect::HtmlOrRedirect};

#[get("/")]
pub fn home(auth_cookie: OptionAuthCookie) -> HtmlOrRedirect {
    if auth_cookie.0.is_some() {
        HtmlOrRedirect::Redirect(Redirect::to("/dashboard"))
    } else {
        HtmlOrRedirect::Html(Html::default()
        .head(
            Head::default().child(
                Meta::default()
                    .attribute(MetaName::Charset)
                    .attribute(Content::new("utf-8")),
            ),
        )
        .body(
            Body::new().apply(ZeroMargin)
            .child(
                Level::new()
                    .child(
                        Level::new().strategy(
                                LayoutStrategy::new()
                                    .axis(LayoutAxis::Horizontal)
                                    .spacing(Spacing::Between)
                            )
                            .child(H1::new("Lovelace").apply(SmallTitle))
                            .child(Div::new()
                                .apply(VerticalAlignCenter)
                                .attribute(Id::new("auth-bar"))
                                .child(A::new().href("/auth/login").text("Login"))
                                .child(Text::new(" "))
                                .child(A::new().href("/auth/register").text("Register"))
                            )
                            .into_div()
                            .apply(compose(YellowBackground, DefaultPadding))
                    )
                    .child(Level::new()
                        .child(P::with_text(
                            "IMPORTANT: This site is in very early stages. Please don't use this
                            for anything serious at this point in time.",
                        ))
                        .child(P::with_text(
                            "Lovelace is a digital platform for learning. It's also quite an
                            incomplete one at the moment, but we're adding features relatively
                            quickly. Updates to this site are rolled out as soon as they're ready,
                            so check back soon for more.",
                        ))
                        .into_div()
                        .apply(compose(GreyBackground, DefaultPadding))
                    )
                    .child(
                        Level::new().child(
                            A::default()
                                .href("https://github.com/lovelace-ed/lovelace")
                                .text("Click me to view the source code.")
                        )
                        .into_div()
                        .apply(DefaultPadding)
                    )
                )
            ))
    }
}
