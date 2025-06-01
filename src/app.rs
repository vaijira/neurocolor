use std::sync::Arc;

use crate::{config::Config, slides::Slides};
use dominator::{Dom, clone, events, html, with_node};
use futures_signals::signal::SignalExt;
use web_sys::HtmlInputElement;

pub struct App {
    config: Arc<Config>,
    slides: Arc<Slides>,
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            config: Config::new(),
            slides: Slides::new(),
        })
    }

    pub fn render(this: Arc<Self>) -> Dom {
        html!("div", {
            .child_signal(this.config.started.signal().map(clone!(this => move |started| {
                if started {
                    Some(Slides::render(this.slides.clone(), this.config.clone()))
                } else {
                    Some(Config::render(this.config.clone()))
                }
            })))
            .child(html!("p", {
            .child(html!("input" => HtmlInputElement, {
              .attr("type", "button")
              .attr("value", "Empezar")
              .with_node!(_element => {
                .event(clone!(this => move |_: events::Click| {
                  this.config.started.set(!this.config.started.get());
                }))
              })
            }))
          }))

        })
    }
}
