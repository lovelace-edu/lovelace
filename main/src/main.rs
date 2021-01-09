#![feature(proc_macro_hygiene, decl_macro)]

use html::{Body, Head, Html, Meta, H1};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Html {
    Html::default()
        .head(
            Head::default()
                .child(
                    Meta::default()
                        .attribute(format!("charset"), format!("UTF-8"))
                        .into(),
                )
                .child(
                    Meta::default()
                        .attribute(format!("lang"), format!("en-GB"))
                        .into(),
                ),
        )
        .body(Body::default().child(H1(format!("Hello World!")).into()))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
