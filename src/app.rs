use std::sync::{Arc, LazyLock};

use dominator::{Dom, class, clone, html};
use futures::StreamExt;
use futures_signals::signal::Mutable;
use gloo_timers::future::IntervalStream;
use rand::Rng;

use crate::config::Config;

static COLORS: [&'static str; 10] = [
    "black", "white", "red", "blue", "green", "orange", "pink", "yellow", "purple", "gray",
];

pub struct App {
    config: Config,
    background_color: Mutable<&'static str>,
    text_color: Mutable<&'static str>,
    text: Mutable<&'static str>,
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            config: Config::new(),
            background_color: Mutable::new(COLORS[1]),
            text_color: Mutable::new(COLORS[0]),
            text: Mutable::new(COLORS[2]),
        })
    }

    fn shuffle(&self) {
        let mut rng = rand::rng();
        let mut numbers = Vec::new();
        while numbers.len() < 3 {
            let num = rng.random_range(0..COLORS.len());
            if !numbers.contains(&num) {
                numbers.push(num);
            }
        }

        self.background_color.replace(COLORS[numbers[0]]);
        self.text_color.replace(COLORS[numbers[1]]);
        self.text.replace(COLORS[numbers[2]]);
    }

    fn render_text(this: Arc<Self>) -> Dom {
        static CENTERED_TEXT_CLASS: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("position", "fixed")
              .style("top", "50%")
              .style("left", "50%")
              .style("transform", "translate(-50%, -50%)")
              .style("font-size", "30vw")
            }
        });

        html!("div", {
          .class(&*CENTERED_TEXT_CLASS)
          .style_signal("color", this.text_color.signal())
          .text_signal(this.text.signal())
        })
    }

    pub fn render(this: Arc<Self>) -> Dom {
        static FULL_CLASS: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("position", "fixed")
              .style("padding", "0")
              .style("margin", "0")
              .style("top", "0")
              .style("left", "0")
              .style("width", "100%")
              .style("height", "100%")
            }
        });

        html!("div", {
          .class(&*FULL_CLASS)
          .future(IntervalStream::new(this.config.interval).for_each(clone!(
            this => move |_| {
              this.shuffle();
              async {}
            }
          )))
          .style_signal("background-color", this.background_color.signal())
          .child(App::render_text(this))
        })
    }
}
