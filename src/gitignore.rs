extern crate hyper;
extern crate hyper_native_tls;

use std::io::Read;
use std::iter::Iterator;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

pub fn get_gitignore(targets: Vec<String>) -> Result<String, hyper::Error> {
    let mut request = "https://www.gitignore.io/api/".to_string();
    for (i, target) in targets.iter().enumerate() {
        request.push_str(&target);

        // skip last ","
        if i != targets.len() - 1 {
            request.push_str(",");
        }
    }

    let ssl = NativeTlsClient::new().unwrap();
    let con = HttpsConnector::new(ssl);
    let client = Client::with_connector(con);
    let url = request.parse::<hyper::Url>().unwrap();
    let req = client.get(url);
    let mut resp = match req.send() {
        Ok(r) => r,
        Err(err) => return Err(err),
    };

    let mut body = String::new();
    resp.read_to_string(&mut body);
    Ok(body)
}
