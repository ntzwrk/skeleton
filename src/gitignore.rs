extern crate hyper;

use std::io::Read;
use std::iter::Iterator;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

/// Fetches a gitignore file from [gitignore.io](https://gitignore.io)
///
/// # Example
///
/// ```
/// mod gitignore
///
/// let gi = gitignore::get_gitignore(&vec!["rust".to_string, "vim".to_string()]).unwrap();
/// ```
pub fn get_gitignore(targets: &[String]) -> Result<String, hyper::Error> {
    let mut request = "https://www.gitignore.io/api/".to_string();
    for (i, target) in targets.iter().enumerate() {
        request.push_str(target);

        // skip last ","
        if i != targets.len() - 1 {
            request.push_str(",");
        }
    }

    let ssl = NativeTlsClient::new().unwrap();
    let con = HttpsConnector::new(ssl);
    let client = Client::with_connector(con);
    let url = request.parse::<hyper::Url>()?;
    let req = client.get(url);
    let mut resp = req.send()?;

    let mut body = String::new();
    resp.read_to_string(&mut body)?;

    Ok(body)
}

#[test]
fn test_get_gitignore() {
    let expected = "
# Created by https://www.gitignore.io/api/rust

### Rust ###
# Generated by Cargo
# will have compiled files and executables
/target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here http://doc.crates.io/guide.html#cargotoml-vs-cargolock
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# End of https://www.gitignore.io/api/rust
";
    let actual = get_gitignore(&vec!["rust".to_string()]).unwrap();
    assert_eq!(expected.to_string(), actual);
}
