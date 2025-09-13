use dotenv_codegen::dotenv;
use http_bytes::http::response::Builder;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod thread;
use thread::ThreadPool;
mod blog;
mod context;
mod database;
mod response;
mod error;

use crate::context::Context;
use crate::response::{build_get_res, build_post_res};

fn main() {
    // Load variables from .env
    let (addr, protocol) = get_addr_protocol();
    let use_https = protocol == "https";

    // Initialize the database
    database::init().unwrap();

    // Build a SSL acceptor from private and public key files
    let acceptor = if use_https {
        let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

        acceptor
            .set_private_key_file(dotenv!("PRIVATE_KEY_FILE"), SslFiletype::PEM)
            .unwrap();
        acceptor
            .set_certificate_chain_file(dotenv!("CERT_CHAIN_FILE"))
            .unwrap();
        acceptor.check_private_key().unwrap();
        Some(Arc::new(acceptor.build()))
    } else {
        None
    };

    // Create a thread pool and use it to handle requests
    let pool = ThreadPool::new(4);

    // Listen on the specified port
    let listener = TcpListener::bind(addr).unwrap();
    println!("listening on {protocol}://{addr}");

    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                if let Some(acceptor) = acceptor.as_ref() {
                    // If we have an SslAcceptor, we should use HTTPS
                    let acceptor = acceptor.clone();
                    pool.execute(move |ctx| match acceptor.accept(tcp_stream) {
                        Ok(ssl_stream) => {
                            if let Err(e) = handle_request(ssl_stream, ctx) {
                                println!("Error handling request: {e}")
                            }
                        }
                        Err(e) => {
                            // This is the important one
                            println!("Error accepting stream: {e}")
                        }
                    });
                } else {
                    // Otherwise, we should use HTTP
                    pool.execute(move |ctx| {
                        if let Err(e) = handle_request(tcp_stream, ctx) {
                            println!("Error handling request: {e}")
                        }
                    })
                }
            }
            Err(e) => {
                println!("Connection failed: {e}");
            }
        }
    }
}

fn handle_request(mut stream: impl Read + Write, ctx: &Context) -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(&mut stream);
    let buf = buf_reader.fill_buf().unwrap_or(&Vec::new()).to_owned();

    let s = std::str::from_utf8(&buf).unwrap_or("");
    println!("{}", s);

    if s.is_empty() {
        println!("received empty or invalid request");
        return Ok(());
    }

    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut req = httparse::Request::new(&mut headers);
    let status = req.parse(&buf)?;

    let res = match req.method {
        Some("GET") => build_get_res(ctx, req, status),
        Some("POST") => {
            if status.is_partial() {
                // If we have a partial status, just continue
                // TODO: this is probably bad
                response::cont()
            } else {
                // Consume a number of bytes from the reader equal to the offset of the body so that we don't
                // repeat the header bytes
                let body_offset = status.unwrap();
                buf_reader.consume(body_offset);

                // Get the Content-Length header, or default to 0 if it can't be read
                let content_length: u64 = req
                    .headers
                    .iter()
                    .find(|h| h.name == "Content-Length")
                    .and_then(|h| std::str::from_utf8(h.value).ok())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);

                // Read a number of bytes equal to the content length into the buffer; this should contain the
                // entire body
                let mut body = Vec::new();
                buf_reader
                    .take(content_length)
                    .read_to_end(&mut body)
                    .unwrap_or(0);

                // Read the body bytes to a &str
                let body = std::str::from_utf8(&body).unwrap_or("");

                // Build a response using the request and the received body
                build_post_res(req, body)
            }
        }
        Some(_) => response::redirect("405", Builder::new(), None),
        None => response::cont(),
    }?;

    stream.write_all(&http_bytes::response_header_to_vec(&res))?;
    stream.write_all(res.body())?;

    Ok(())
}

fn get_addr_protocol() -> (&'static str, &'static str) {
    if is_debug() {
        (dotenv!("DEBUG_ADDR"), dotenv!("DEBUG_PROTOCOL"))
    } else {
        (dotenv!("RELEASE_ADDR"), dotenv!("RELEASE_PROTOCOL"))
    }
}

#[cfg(debug_assertions)]
fn is_debug() -> bool {
    return true;
}

#[cfg(not(debug_assertions))]
fn is_debug() -> bool {
    return false;
}
