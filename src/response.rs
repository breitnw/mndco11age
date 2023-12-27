use std::collections::HashMap;
use http_bytes::http::{Error, header, Method, Response};
use minijinja::context;
use std::fs;
use std::io::Read;
use dotenv_codegen::dotenv;
use http_bytes::http::response::Builder;
use crate::blog::Article;
use crate::Context;
use crate::database as db;

/// Builds an HTTP response to a given request. returns an Err() if a response cannot be built from the response builder;
/// i.e., if `res_builder.body(_)` fails.
///
/// # Panics
/// - If the URL cannot be parsed to determine query pairs. This should never happen, but keep an eye out for bugs.
/// - If the list of guests cannot be added to or retrieved
pub(crate) fn build_res(
    ctx: &Context,
    buf: &[u8],
) -> Result<Response<Vec<u8>>, Error> {
    let mut res_builder = Response::builder();

    // If the request is complete, parse the header; otherwise, return a continue status code
    let (req, mut body_bytes) = if let Ok(Some(val)) = http_bytes::parse_request_header_easy(buf) {
        val
    } else {
        return res_builder
            .status(100)
            .body(vec![]);
    };

    let mut body = String::new();
    body_bytes.read_to_string(&mut body).unwrap();

    // Get the parts of the URI path, split at the "/" separator
    let path = req.uri().path();
    let path_split: Vec<_> = path.split("/").skip(1).collect();

    // Switch on the url to determine which HTML template to return
    match req.method() {
        &Method::GET => {
            let (template, context) = match path_split[0] {
                "static" => {
                    // If the first part is "static", access a static resource
                    let mime_type = new_mime_guess::from_path(req.uri().path()).first_or(mime::TEXT_HTML);
                    if let Ok(file_contents) = fs::read(&path[1..]) {
                        return res_builder
                            .header(header::CONTENT_TYPE, mime_type.essence_str())
                            .header(header::CONTENT_LENGTH, file_contents.len())
                            .body(file_contents);
                    } else {
                        // Return 404 if the resource is unavailable
                        res_builder.status(404);
                        ("404.html", context! {})
                    }
                }
                "" => {
                    let cards = &ctx.cards;
                    let column_length = (cards.len() + 1) / 3;

                    let ctx = context! {
                columns => vec![
                    &cards[..column_length],
                    &cards[column_length..2 * column_length],
                    &cards[2 * column_length..]
                ]
            };
                    ("software.html", ctx)
                },
                "music" => ("music.html", context! {}),
                "blog" => {
                    match path_split.get(1).map(|location| db::get_article(location)) {
                        // If there is a valid second part to the path, get the corresponding article
                        Some(Ok(article)) => ("blog-post.html", context! { article => article }),
                        // If there's a second part but it's invalid, return 404
                        Some(Err(_)) => {
                            res_builder.status(404);
                            ("404.html", context! {})
                        }
                        // If there's not a second part, return the blog homepage
                        None => ("blog.html", context! { articles => db::get_articles().unwrap() })
                    }
                },
                "blog-add" => ("blog-add.html", context! {}),
                "guestbook" => {
                    // Otherwise, render the page
                    let sign_disabled = req.headers()
                        .get("cookie")
                        .and_then(|cookie| cookie.to_str().ok())
                        .map_or(false, |cookie| cookie.contains("sign-disabled=true"));
                    ("guestbook.html", context! {
                        guests => db::get_guests().unwrap(),
                        sign_disabled => sign_disabled
                    })
                }
                _ => {
                    res_builder.status(404);
                    ("404.html", context! {})
                }
            };

            let body_result = ctx.jinja_env
                .get_template(template)
                .and_then(|t| t.render(context))
                .map(|s| s.into_bytes());
            let body = body_result.unwrap_or(
                // respond with 404 if we have some issue getting the HTML
                ctx.jinja_env
                    .get_template("404.html")
                    .unwrap()// Ok to unwrap here because 404.html always exists
                    .render(context! {})
                    .unwrap()
                    .into_bytes());

            return res_builder
                .header(header::CONTENT_TYPE, mime::TEXT_HTML.essence_str())
                .header(header::CONTENT_LENGTH, body.len())
                .body(body);
        }
        &Method::POST => {
            let post_map = parse_pairs(&body);
            match path_split[0] {
                "guestbook" => {
                    // If we're POSTing, add the user's name to the guestbook and set a cookie to indicate that they
                    // already signed
                    if let Some(name) = post_map.get("name") {
                        db::add_guest(name).unwrap();
                        res_builder.header(header::SET_COOKIE, "sign-disabled=true");
                        return redirect("/guestbook", res_builder)
                    }
                    redirect("/guestbook", res_builder)
                }
                "blog-add" => {
                    if let (Some(title), Some(markdown), Some(key)) = (post_map.get("title"), post_map.get("markdown"), post_map.get("key")) {
                        if key == dotenv!("SUPER_SECRET_KEY") {
                            db::add_article(Article::new(title, markdown)).unwrap();
                        }
                    }
                    redirect("/blog", res_builder)
                }
                _ => redirect("/404", res_builder)
            }
        }
        _ => redirect("/404", res_builder) // TODO: should technically be 405 i think
    }
}

/// Returns a redirect request to the specified endpoint
fn redirect(endpoint: &str, mut builder: Builder) -> Result<Response<Vec<u8>>, Error> {
    builder
        .status(303)
        .header(header::LOCATION, endpoint)
        .body(vec![])
}

/// Parses a list of percent-encoded pairs into a HashMap of keys and values. Instead of returning a Result, simply omits
/// pairs that produce a parsing error
fn parse_pairs(str: &str) -> HashMap<String, String> {
    // Use the Url crate to parse query pairs and POST pairs
    let mut pairs = HashMap::new();
    for pair in str.replace("+", "%20").split("&") {
        let mut pair: Vec<String> = pair
            .split("=")
            .map(|s| percent_encoding::percent_decode_str(s).decode_utf8_lossy().into_owned())
            .collect();
        if let (Some(value), Some(key)) = (pair.pop(), pair.pop()) {
            pairs.insert(key, value);
        }
    }
    pairs
}