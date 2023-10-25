use http_bytes::http::{header, response, Response};
use minijinja::{context, Environment};
use std::error::Error;
use std::fs;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use serde_xml_rs::from_str;
use include_dir::{include_dir, Dir};

// TODO: put this on another page or in a xml file
#[derive(Serialize, Deserialize, Debug)]
struct Card {
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

pub(crate) fn build_res(
    env: Arc<Environment>,
    buf: &[u8],
) -> Result<(response::Builder, Vec<u8>), Box<dyn Error>> {
    let mut res_builder = Response::builder();
    let mut body: Vec<u8> = Vec::new();

    let (req, _req_body) = if let Ok(Some(val)) = http_bytes::parse_request_header_easy(buf) {
        val
    } else {
        res_builder.status(100);
        return Ok((res_builder, body));
    };

    let uri = req.uri().path();

    if uri.starts_with("/static/") {
        let mime_type = new_mime_guess::from_path(uri).first_or(mime::TEXT_HTML);
        if let Ok(file_contents) = fs::read(&uri[1..]) {
            body = file_contents;
            res_builder
                .header(header::CONTENT_TYPE, mime_type.essence_str())
                .header(header::CONTENT_LENGTH, body.len());
        }
    } else {
        let (template, context) = match uri {
            "/" => {
                // TODO: Don't read the file every time the page is visited
                // let mut file = File::open("../data/data.xml").expect("Unable to open the file");
                // let mut contents = String::new();
                // file.read_to_string(&mut contents).expect("Unable to read the file");
                // println!("{}", contents);

                const DATA_DIR: Dir = include_dir!("data");
                let data = DATA_DIR.get_file("data.xml").unwrap().contents_utf8().unwrap();
                let cards: Cards = from_str(data).unwrap();
                let cards = cards.cards;

                let column_length = (cards.len() + 1) / 3;

                let ctx = context! {
                    columns => vec![
                        &cards[..column_length],
                        &cards[column_length..2 * column_length],
                        &cards[2 * column_length..]
                    ]
                };
                ("index.html", ctx)
            },
            "/empty" => ("empty.html", context! {}),
            "/demo" => ("demo.html", context! {}),
            _ => {
                res_builder.status(404);
                ("404.html", context! {})
            }
        };
        body = env.get_template(template)?.render(context)?.into_bytes();

        res_builder
            // .header(header::CACHE_CONTROL, )
            .header(header::CONTENT_TYPE, mime::TEXT_HTML.essence_str())
            .header(header::CONTENT_LENGTH, body.len());
    }

    Ok((res_builder, body))
}
