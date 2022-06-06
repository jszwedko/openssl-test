use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslStream, SslVerifyMode};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file("localhost.key", SslFiletype::PEM)
        .unwrap();
    acceptor
        .set_certificate_file("localhost.crt", SslFiletype::PEM)
        .unwrap();
    acceptor.set_verify(SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT);

    acceptor.set_ca_file("Vector_CA.crt").unwrap();
    acceptor.set_verify_callback(SslVerifyMode::PEER, |res, cert| {
        println!(
            "{}",
            std::str::from_utf8(&cert.current_cert().unwrap().to_text().unwrap()).unwrap()
        );
        dbg!(cert.current_cert().unwrap().subject_name());
        dbg!(cert.current_cert().unwrap().issuer_name());
        res
    });
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());

    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();

    fn handle_client(stream: SslStream<TcpStream>) {
        // ...
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    handle_client(stream);
                });
            }
            Err(e) => {
                panic!("{}", e)
            }
        }
    }
}
