use http_bytes::http::{header, response, Response};
use minijinja::{context, Environment};
use std::error::Error;
use std::fs;
use std::sync::Arc;

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
            "/" => ("index.html", context! {}),
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
