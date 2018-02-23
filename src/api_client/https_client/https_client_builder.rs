
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;

pub struct HttpsClientBuilder;

impl HttpsClientBuilder {
    
    pub fn build() -> Client {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        Client::with_connector(connector)
    }
}