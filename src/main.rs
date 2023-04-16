mod multithreading;

use std::error::Error;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*, BufReader};
use std::sync::Arc;
use http_bytes::http::{header, Response};
use http_bytes::http::response::Builder;

use minijinja::{context, Environment, Source};

use new_mime_guess;
use mime;

use openssl::ssl::SslAcceptor;
use openssl::ssl::SslFiletype;
use openssl::ssl::SslMethod;

use multithreading::ThreadPool;
use openssl::ssl::SslStream;

fn main() {
    const ADDR: &str = "0.0.0.0:443";

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor.set_private_key_file("/etc/letsencrypt/live/mndco11age.xyz/privkey.pem", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file("/etc/letsencrypt/live/mndco11age.xyz/fullchain.pem").unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());

    let listener = TcpListener::bind(ADDR).unwrap();

    println!("listening on {ADDR}");

    // Add minijinja templates to the environment
    let mut env = Environment::new();
    env.set_source(Source::from_path("templates"));

    let env = Arc::from(env);

    // Create a thread pool and use it to handle requests
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                let env = env.clone();
                
                pool.execute(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    handle_connection(stream, env).unwrap();
                });
            }
            Err(_) => {
                println!("Connection failed!");
            },
        }
    }
}

fn handle_connection(
    mut stream: SslStream<TcpStream>, 
    env: Arc<Environment>
) -> Result<(), Box<dyn Error>> {
    stream.do_handshake()?;

    let mut buf_reader = BufReader::new(&mut stream);
    let buf = buf_reader.fill_buf().unwrap();

    let (mut res_builder, body) = build_res(env, buf)?;

    let res = res_builder.body(())?.into();
    let bytes = http_bytes::response_header_to_vec(&res);

    stream.write_all(&bytes)?;
    stream.write_all(&body)?;

    Ok(())
}

fn build_res(env: Arc<Environment>, buf: &[u8]) -> Result<(Builder, Vec<u8>), Box<dyn Error>>  {
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
            "/" => ("index.html", context!{}),
            "/empty" => ("empty.html", context!{}),
            "/demo" => ("demo.html", context!{}),
            _ => {
                res_builder.status(404);
                ("404.html", context!{})
            }
        };
        body = env
            .get_template(template)?
            .render(context)?
            .into_bytes();

        res_builder
            .header(header::CONTENT_TYPE, mime::TEXT_HTML.essence_str())
            .header(header::CONTENT_LENGTH, body.len());
    }

    Ok((res_builder, body))
}