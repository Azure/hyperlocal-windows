extern crate futures;
extern crate hyper;
extern crate hyperlocal_windows;

use hyper::service::service_fn;
use hyper::{header, Body, Request, Response};
use std::{fs, io};

const PHRASE: &'static str = "It's a Unix system. I know this.";

fn hello(
    req: Request<Body>,
) -> impl futures::Future<Item = Response<Body>, Error = io::Error> + Send {
    println!("servicing new request {:?}", req);
    futures::future::ok(
        Response::builder()
            .header(header::CONTENT_TYPE, "text/plain")
            .header(header::CONTENT_LENGTH, PHRASE.len() as u64)
            .body(PHRASE.into())
            .expect("failed to create response"),
    )
}

fn run() -> io::Result<()> {
    if let Err(err) = fs::remove_file("test.sock") {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err);
        }
    }

    let svr = hyperlocal_windows::server::Server::bind("test.sock", || service_fn(hello))?;

    {
        let path = svr.local_addr().as_pathname().unwrap();
        println!(
            "Listening on unix://{path} with 1 thread.",
            path = path.to_string_lossy(),
        );
    }

    svr.run()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error starting server: {}", err)
    }
}
