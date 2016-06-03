extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate hyper;

use std::str::FromStr;
use std::net::IpAddr;

use ansi_term::Colour::{Red, Green};
use clap::{App, Arg};
use hyper::Url;
use hyper::error::Result;
use hyper::header::{Location, Server as ServerHeader};
use hyper::server::{Server, Listening, Request, Response};
use hyper::status::StatusCode::PermanentRedirect;
use hyper::uri::RequestUri::AbsolutePath;


static SERVER_NAME: &'static str = "redirect";


fn start_server(ip: IpAddr, port: u16, new_host: Url) -> Result<Listening> {
    try!(Server::http((ip, port))).handle(move |req: Request, mut res: Response| {
        let mut new_url = new_host.clone();

        let path = match req.uri {
            AbsolutePath(path) => path,
            _ => "".to_string(),
        };
        new_url.set_path(&path);

        res.headers_mut().set(Location(new_url.to_string()));
        res.headers_mut().set(ServerHeader(SERVER_NAME.to_string()));
        *res.status_mut() = PermanentRedirect;
        res.send(b"").unwrap();
    })
}


fn main() {
    let matches = App::new("redirect")
                      .version("0.1")
                      .author("Rocky Meza <rockymeza@gmail.com>")
                      .about("Simple redirecting HTTP server.")
                      .arg(Arg::with_name("NEW_HOST")
                               .help("Host to redirect to (http://example.com)")
                               .required(true)
                               .index(1))
                      .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .value_name("PORT")
                               .help("Port to listen on (default: 80)")
                               .takes_value(true))
                      .get_matches();

    let ip = IpAddr::from_str("0.0.0.0").unwrap();
    let port = value_t!(matches, "port", u16).unwrap_or(80);
    let new_host = value_t!(matches, "NEW_HOST", Url).unwrap_or_else(|e| e.exit());

    match start_server(ip, port, new_host.clone()) {
        Ok(listener) => {
            println!("{} {} => {}",
                     Green.paint("redirect:"),
                     listener.socket,
                     new_host,
                     );
        }
        Err(err) => {
            println!("{} {}", Red.paint("error:"), err);
        }
    };
}
