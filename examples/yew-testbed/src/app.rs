// Copyright © 2020 Lukas Wagner

extern crate css_in_rust;

use css_in_rust::style::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    style: Style,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let style = match Style::create(
            String::from("App"),
            String::from(
                r#"
                background-color: darkred;
                padding: 5px;
                &:hover {
                    background-color: red;
                }

                .on-da-inside {
                    background-color: blue;
                    width: 100px;
                    color: #ddd;
                    padding: 5px;
                    animation: move 2s infinite;
                    animation-timing-function: linear;
                    animation-direction: alternate;
                }

                @keyframes move {
                    from {
                        width: 100px;
                    }
                    to {
                        width: 200px;
                    }
                }

                @media only screen and (min-width: 626px) {
                    width: 600px;
                    margin: auto;

                    @keyframes move {
                        from {
                            width: 100px;
                        }
                        to {
                            width: 590px;
                        }
                    }
                }
                "#,
            ),
        ) {
            Ok(style) => style,
            Err(error) => {
                panic!("An error occured while creating the style: {}", error);
            }
        };
        App { style }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {<div class=self.style.clone()>
            {"The quick brown fox jumps over the lazy dog"}
            <div class="on-da-inside">{"The quick brown fox jumps over the lazy dog"}</div>
        </div>}
    }
}
