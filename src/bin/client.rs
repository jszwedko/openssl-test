use openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslStream, SslVerifyMode};
use std::net::TcpStream;

#[tokio::main]
async fn main() {
    let server_addr = "localhost:9000";

    let connection = TcpStream::connect(&server_addr).unwrap();
    let ssl = {
        let mut connector = SslConnector::builder(SslMethod::tls_client()).unwrap();
        connector
            .set_private_key_file("tls_meta_client.key", SslFiletype::PEM)
            .unwrap();
        connector
            .set_certificate_file("tls_meta_client.crt", SslFiletype::PEM)
            .unwrap();
        //connector
        //.set_ca_file("Vector_CA.crt")
        //.unwrap();
        connector.set_verify(SslVerifyMode::NONE);
        connector
            .build()
            .configure()
            .unwrap()
            .into_ssl(&server_addr)
            .unwrap()
    };
    let mut connection = SslStream::new(ssl, connection).unwrap();

    connection.connect().unwrap();
}
