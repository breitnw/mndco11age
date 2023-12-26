use dotenv_codegen::dotenv;
use include_dir::{Dir, include_dir};
use minijinja::{Environment, Source};
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use crate::{get_addr_protocol};

pub(crate) struct Context<'a> {
    pub(crate) cards: Vec<Card>,
    pub(crate) jinja_env: Environment<'a>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        let (_, protocol) = get_addr_protocol();

        // Add minijinja templates to the environment
        let mut jinja_env = Environment::new();
        jinja_env.set_source(Source::from_path("templates"));
        jinja_env.add_global("BASE_URL", format!("{}://{}", protocol, dotenv!("STATIC_HOST")));

        // Parse data for cards
        const DATA_DIR: Dir = include_dir!("data");
        let data = DATA_DIR.get_file("card_data.xml").unwrap().contents_utf8().unwrap();
        let cards: Cards = from_str(data).unwrap();
        let cards = cards.cards;

        Context { cards, jinja_env }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    link: String,
    image_src: String,
    name: String,
    description: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'static"))]
struct Cards {
    #[serde(rename = "$value")]
    cards: Vec<Card>
}