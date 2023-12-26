extern crate dotenv_codegen;

use dotenv_codegen::dotenv;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod thread;
use thread::ThreadPool;

mod response;
mod database;
mod context;

use response::build_res;
use crate::context::Context;


fn main() {
    // TODO: enable caching
    // TODO: add URL to .env for links/resources, load into minijinja environment

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

    for tcp_stream in listener.incoming() { match tcp_stream {
        Ok(tcp_stream) => {
            if let Some(acceptor) = acceptor.as_ref() {
                // If we have an SslAcceptor, we should use HTTPS
                let acceptor = acceptor.clone();
                pool.execute(move |ctx|
                    match acceptor.accept(tcp_stream) {
                        Ok(ssl_stream) => {
                            if let Err(e) = handle_request(ssl_stream, ctx) {
                                println!("Error handling connection: {}", e)
                            }
                        }
                        Err(e) => {
                            println!("Error accepting stream: {}", e)
                        }
                    }
                );
            } else {
                // Otherwise, we should use HTTP
                pool.execute(move |ctx| if let Err(e) = handle_request(tcp_stream, ctx) {
                    println!("Error handling connection: {}", e)
                })
            }
        }
        Err(_) => {
            println!("Connection failed!");
        }
    }
}}

fn handle_request(
    mut stream: impl Read + Write,
    ctx: &Context,
) -> Result<(), Box<dyn Error>> {
    let mut buf_reader = BufReader::new(&mut stream);
    let buf = buf_reader.fill_buf().unwrap();

    let res = build_res(ctx, buf)?;

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
