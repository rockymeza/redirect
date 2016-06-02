extern crate hyper;

use std::str::FromStr;
use std::net::IpAddr;

use hyper::Url;
use hyper::header::Location;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode::PermanentRedirect;
use hyper::uri::RequestUri::AbsolutePath;


fn main() {
    let new_host = Url::parse("http://example.com/").unwrap();
    let ip = IpAddr::from_str("0.0.0.0").unwrap();
    let port = 8000;

    Server::http((ip, port))
        .unwrap()
        .handle(move |req: Request, mut res: Response| {
            let mut new_url = new_host.clone();
            let path = match req.uri {
                AbsolutePath(path) => path,
                _ => "".to_string(),
            };
            new_url.set_path(&path);

            res.headers_mut().set(Location(new_url.to_string()));
            *res.status_mut() = PermanentRedirect;
            res.send(b"").unwrap();
        })
        .unwrap();
}
