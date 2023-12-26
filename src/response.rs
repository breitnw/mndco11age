use std::borrow::Cow;
use http_bytes::http::{Error, header, Response, response};
use minijinja::context;
use std::fs;
use http_bytes::http::response::Builder;
use url::Url;
use crate::Context;
use crate::database::{add_guest, get_guests};

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
    let (req, _req_body) = if let Ok(Some(val)) = http_bytes::parse_request_header_easy(buf) {
        val
    } else {
        return res_builder
            .status(100)
            .body(vec![]);
    };

    // Get the parts of the URI path, split at the "/" separator
    let path = req.uri().path();
    let path_split: Vec<_> = path.split("/").skip(1).collect();

    // Use the Url crate to parse query pairs
    let url = Url::parse(&format!("https://mndco11age.xyz{}", &req.uri().to_string())).unwrap();
    let query_pairs = url.query_pairs().collect::<Vec<_>>();

    // Switch on the url to determine which HTML template to return
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
        "blog" => ("blog.html", context! {}),
        "guestbook" => {
            // If there's a query, add the user's name to the guestbook and set a cookie to indicate that they
            // already signed
            if let Some((Cow::Borrowed("name"), name)) = query_pairs.get(0) {
                add_guest(name).unwrap();
                res_builder.header(header::SET_COOKIE, "sign-disabled=true");
                return redirect("/guestbook", res_builder)
            }
            // Otherwise, render the page
            let sign_disabled = req.headers()
                .get("cookie")
                .and_then(|cookie| cookie.to_str().ok())
                .map_or(false, |cookie| cookie.contains("sign-disabled=true"));
            ("guestbook.html", context! {
                guests => get_guests().unwrap(),
                sign_disabled => sign_disabled
            })
        },
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

/// Returns a redirect request to the specified endpoint
fn redirect(endpoint: &str, mut builder: Builder) -> Result<Response<Vec<u8>>, Error> {
    builder
        .status(303)
        .header(header::LOCATION, endpoint)
        .body(vec![])
}