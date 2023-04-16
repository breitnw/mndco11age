use std::fs::File;

use openssl::{self, pkey::PKey, encrypt::{Encrypter, Decrypter}, ssl::{SslContextBuilder, SslMethod, SslFiletype, SslVerifyMode, SslAcceptor}};

struct SslManager {
    
}

impl SslManager {
    fn new() {
        let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        acceptor.set_private_key_file("/etc/letsencrypt/live/mndco11age.xyz/privkey.pem", SslFiletype::PEM).unwrap();
        acceptor.set_certificate_chain_file("/etc/letsencrypt/live/mndco11age.xyz/fullchain.pem").unwrap();
        acceptor = Arc::new(acceptor.build())
    }
}