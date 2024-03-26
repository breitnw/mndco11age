use crate::blog::Article;
use crate::database as db;
use crate::Context;
use dotenv_codegen::dotenv;
use http_bytes::http::response::Builder;
use http_bytes::http::{header, Error, Response};
use minijinja::{context, Environment};
use std::collections::HashMap;
use std::fs;
use crate::error::HttpError;
use crate::error::HttpError::{ClientError, ServerError};

/// Builds an HTTP response to a given request. returns an Err() if a response cannot be built from the response builder;
/// i.e., if `res_builder.body(_)` fails.
///
/// # Panics
/// - If the URL cannot be parsed to determine query pairs. This should never happen, but keep an eye out for bugs.
/// - If the list of guests cannot be added to or retrieved
pub(crate) fn build_get_res(
    ctx: &Context,
    req: httparse::Request,
    status: httparse::Status<usize>,
) -> Result<Response<Vec<u8>>, Error> {
    let mut res_builder = Builder::new();

    let path = {
        if let Some(path) = req.path {
            path
        } else {
            return cont();
        }
    };
    
    // separate path and query
    let (path, query) = path.split_once("?").unwrap_or((path, ""));
    let query = parse_pairs(query);
    let err = query.get("err");
    
    
    // Get the parts of the URI path, split at the "/" separator
    let path_split: Vec<_> = path.split("/").skip(1).collect();

    let (template, page_context) = match path_split[0] {
        "static" => {
            // If the first part is "static", access a static resource
            let mime_type = new_mime_guess::from_path(path).first_or(mime::TEXT_HTML);
            if let Ok(file_contents) = fs::read(&path[1..]) {
                return res_builder
                    .header(header::CONTENT_TYPE, mime_type.essence_str())
                    .header(header::CONTENT_LENGTH, file_contents.len())
                    .body(file_contents);
            } else {
                // Return 404 if the resource is unavailable
                error(404, path, &mut res_builder)
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
        }
        "music" => ("music.html", context! {}),
        "blog" => {
            match path_split.get(1).map(|location| db::get_article(location)) {
                // If there is a valid second part to the path, get the corresponding article
                Some(Ok(article)) => (
                    "blog-post.html",
                    context! { article => article },
                ),
                // If there's a second part but it's invalid, return 404
                Some(Err(_)) => error(404, path, &mut res_builder),
                // If there's not a second part, return the blog homepage
                None => (
                    "blog.html",
                    context! { articles => db::get_articles().unwrap()},
                ),
            }
        }
        "blog-add" => ("blog-add.html", context! { path => path }),
        "guestbook" => {
            if status.is_partial() {
                // If we have a partial response we might not have the cookies so we should wait
                return cont();
            }
            let sign_disabled: bool = {
                if let Some(&cookie) = req.headers.iter().find(|h| h.name == "Cookie") {
                    let cookie_str = std::str::from_utf8(cookie.value);
                    cookie_str.is_ok_and(|s| s.contains("sign-disabled=true"))
                } else {
                    false
                }
            };
            (
                "guestbook.html",
                context! {
                    guests => db::get_guests().unwrap(),
                    sign_disabled => sign_disabled,
                },
            )
        }
        "404" | "405" | "500" => {
            let status: u16 = path_split[0].parse().unwrap();
            error(status, path, &mut res_builder)
        }
        _ => error(404, path, &mut res_builder),
    };
    
    let jinja_context = context! {
        page_context => page_context,
        path => path,
        err => err
    };
    
    let body = to_bytes(template, jinja_context, &ctx.jinja_env).unwrap_or_else(|_| {
        // We need to use a closure since error mutates state
        let (template, context) = error(500, path, &mut res_builder);
        to_bytes(template, context, &ctx.jinja_env).unwrap()
    });
    return res_builder
        .header(header::CONTENT_TYPE, mime::TEXT_HTML.essence_str())
        .header(header::CONTENT_LENGTH, body.len())
        .body(body);
}

pub(crate) fn build_post_res(
    req: httparse::Request,
    body: &str,
) -> Result<Response<Vec<u8>>, Error> {
    let mut res_builder = Builder::new();

    let path = {
        if let Some(path) = req.path {
            path
        } else {
            return cont();
        }
    };

    // Get the parts of the URI path, split at the "/" separator
    let path_split: Vec<_> = path.split("/").skip(1).collect();

    let post_map = parse_pairs(&body);
    match path_split[0] {
        "guestbook" => {
            // If we're POSTing, add the user's name to the guestbook and set a cookie to indicate that they
            // already signed
            if let Some(name) = post_map.get("name") {
                if name.len() > 100 {
                    // we don't allow messages over 100 characters
                    return redirect("/guestbook", res_builder, Some(ClientError("your message is too long! please keep it at 100 characters or less.")));
                }
                match db::add_guest(name) {
                    Ok(()) => {
                        res_builder.header(header::SET_COOKIE, "sign-disabled=true");
                        redirect("/guestbook", res_builder, None)
                    }
                    Err(e) => {
                        redirect("/guestbook", res_builder, Some(ServerError(&e.to_string())))
                    }
                }
                
            } else {
                println!("failed guestbook submission with body: {}", body);
                redirect("/guestbook", res_builder, Some(ServerError("guestbook submission failed, not all required fields provided...")))
            }
        }
        "blog-add" => {
            if let (Some(title), Some(tagline), Some(markdown), Some(key)) = (
                post_map.get("title"),
                post_map.get("tagline"),
                post_map.get("markdown"),
                post_map.get("key"),
            ) {
                if key == dotenv!("SUPER_SECRET_KEY") {
                    db::add_article(Article::new(title, tagline, markdown)).unwrap();
                }
                redirect("/blog", res_builder, None)
            } else {
                println!("failed blog-add submission with body: {}", body);
                redirect("/blog-add", res_builder, Some(ServerError("guestbook submission failed, not all required fields provided...")))
            }
        }
        _ => redirect("404", res_builder, None),
    }
}

/// Returns a redirect request to the specified endpoint
pub(crate) fn redirect<'a>(
    endpoint: &str, 
    mut builder: Builder, 
    err: Option<HttpError<'a>>
) -> Result<Response<Vec<u8>>, Error> {
    let url = match err {
        Some(e) => format!("{}?{}", endpoint, e.to_query()),
        _ => endpoint.to_owned()
    };
    builder
        .status(303)
        .header(header::LOCATION, url)
        .body(vec![])
}

/// Returns a continue status with an empty body
pub(crate) fn cont() -> Result<Response<Vec<u8>>, Error> {
    return Response::builder().status(100).body(vec![]);
}

/// Returns a template name and a minijinja context representing an error status. Supports 404 (page not found),
/// 405 (method not supported), and 500 (internal server error)
fn error(
    status: u16,
    path: &str,
    builder: &mut Builder,
) -> (&'static str, minijinja::value::Value) {
    builder.status(status);
    let message = match status {
        404 => "there's not really anything here at the moment",
        405 => "method not allowed",
        500 => "internal server error",
        _ => "something went wrong",
    };
    (
        "error.html",
        context! {
            path => path,
            status => status,
            message => message
        },
    )
}

/// Converts a template name and minijinja context to HTML bytes
fn to_bytes(
    template: &str,
    context: minijinja::value::Value,
    jinja_env: &Environment,
) -> Result<Vec<u8>, minijinja::Error> {
    jinja_env
        .get_template(template)
        .and_then(|t| t.render(context))
        .map(|s| s.into_bytes())
}

/// Parses a list of percent-encoded pairs into a HashMap of keys and values. Instead of returning a Result, simply omits
/// pairs that produce a parsing error
fn parse_pairs(str: &str) -> HashMap<String, String> {
    // Use the Url crate to parse query pairs and POST pairs
    let mut pairs = HashMap::new();
    for pair in str.replace("+", "%20").split("&") {
        let mut pair: Vec<String> = pair
            .split("=")
            .map(|s| {
                percent_encoding::percent_decode_str(s)
                    .decode_utf8_lossy()
                    .into_owned()
            })
            .collect();
        if let (Some(value), Some(key)) = (pair.pop(), pair.pop()) {
            pairs.insert(key, value);
        }
    }
    pairs
}
