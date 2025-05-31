use std::sync::{Arc, LazyLock};

use dominator::{Dom, class, clone, html};
use futures::StreamExt;
use futures_signals::signal::Mutable;
use gloo_timers::future::IntervalStream;
use rand::Rng;
use web_sys::HtmlAudioElement;

use crate::config::Config;

static COLORS: [&'static str; 10] = [
    "black", "white", "red", "blue", "green", "orange", "pink", "yellow", "purple", "gray",
];

static ES_COLORS: [&'static str; 10] = [
    "NEGRO", "BLANCO", "ROJO", "AZUL", "VERDE", "NARANJA", "ROSA", "AMARILLO", "VIOLETA", "GRIS",
];

pub struct App {
    config: Config,
    background_audio: HtmlAudioElement,
    word_audio: HtmlAudioElement,
    letters_audio: HtmlAudioElement,
    background_color: Mutable<&'static str>,
    text_color: Mutable<&'static str>,
    text: Mutable<&'static str>,
}

impl App {
    pub fn new() -> Arc<Self> {
        let background_audio = HtmlAudioElement::new_with_src("audio/fondo.ogg").unwrap();
        // background_audio.set_autoplay(true);
        let word_audio = HtmlAudioElement::new_with_src("audio/palabra.ogg").unwrap();
        // word_audio.set_autoplay(true);
        let letters_audio = HtmlAudioElement::new_with_src("audio/letras.ogg").unwrap();
        // letters_audio.set_autoplay(true);

        Arc::new(Self {
            config: Config::new(),
            background_audio,
            word_audio,
            letters_audio,
            background_color: Mutable::new(COLORS[1]),
            text_color: Mutable::new(COLORS[0]),
            text: Mutable::new(ES_COLORS[2]),
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
        self.text.replace(ES_COLORS[numbers[2]]);

        match rng.random_range(0..3) {
            0 => {
                let _ = self.background_audio.play();
            }
            1 => {
                let _ = self.word_audio.play();
            }
            2 => {
                let _ = self.letters_audio.play();
            }
            _ => log::warn!("Unexpected value for playing audio. It should never happen."),
        }
    }

    fn render_text(this: Arc<Self>) -> Dom {
        static CENTERED_TEXT_CLASS: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("position", "fixed")
              .style("top", "50%")
              .style("left", "50%")
              .style("transform", "translate(-50%, -50%)")
              .style("font-size", "15vw")
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
