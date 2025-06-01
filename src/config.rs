use std::sync::Arc;

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::signal::Mutable;
use web_sys::HtmlInputElement;

pub struct Config {
    pub started: Mutable<bool>,
    pub slides: Mutable<u32>,
    pub interval: Mutable<u32>,
}

impl Config {
    const SLIDES_DEFAULT_VALUE: u32 = 10;
    const INTERVAL_DEFAULT_VALUE: u32 = 3;
    const MS_IN_SECOND: u32 = 1000;

    pub fn new() -> Arc<Self> {
        Arc::new(Config {
            started: Mutable::new(false),
            slides: Mutable::new(Config::SLIDES_DEFAULT_VALUE),
            interval: Mutable::new(Config::INTERVAL_DEFAULT_VALUE * Config::MS_IN_SECOND),
        })
    }

    pub fn render(this: Arc<Self>) -> Dom {
        html!("div", {
            .child(html!("h1", {
              .text("NEUROCOLOR")
            }))
            .child(html!("p", {
              .text("Mostrar ")
              .child(html!("input" => HtmlInputElement, {
                .attr("type", "number")
                .attr("value", &this.slides.get().to_string())
                .attr("min", "1")
                .attr("max", "99")
                .style("width", "40px")
                .with_node!(element => {
                  .event(clone!(this => move |_: events::Change| {
                    this.slides.set(element.value().parse::<u32>().unwrap_or(Config::SLIDES_DEFAULT_VALUE));
                  }))
                })
              }))
              .text(" diapositivas en total cambiando cada ")
              .child(html!("input" => HtmlInputElement, {
                .attr("type", "number")
                .attr("value", &(this.interval.get() / Config::MS_IN_SECOND).to_string())
                .attr("min", "1")
                .attr("max", "60")
                .style("width", "40px")
                .with_node!(element => {
                  .event(clone!(this => move |_: events::Change| {
                    this.interval.set(element.value().parse::<u32>().unwrap_or(Config::INTERVAL_DEFAULT_VALUE) * Config::MS_IN_SECOND);
                  }))
                })
              }))
              .text(" segundos.")
            }))
        })
    }
}
