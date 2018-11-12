> [hyper](https://github.com/hyperium/hyper) client and server bindings for [unix domain sockets](https://github.com/Azure/tokio-uds-windows) on Windows

Hyper is a rock solid [rustlang](https://www.rust-lang.org/) HTTP client and server tool kit. [Unix domain sockets](https://en.wikipedia.org/wiki/Unix_domain_socket) provide
a mechanism for host-local interprocess communication. Hyperlocal builds on and complements hyper's interfaces for building unix domain socket HTTP clients and servers.

This is useful for exposing simple HTTP interfaces for your Unix daemons in cases where you want to limit access to the current host, in which case, opening and exposing tcp ports is not needed. Examples of unix daemons that provide this kind of host local interface include, [docker](https://docs.docker.com/engine/misc/), a process container manager.


## install

Add the following to your `Cargo.toml` file

```toml
[dependencies]
hyperlocal-windows = "0.1.0"
```

## usage

### servers

A typical server can be built with `hyperlocal_windows::server::Server`

```rust
extern crate hyper;
extern crate hyperlocal_windows;
extern crate futures;

use std::fs;
use std::io;

use hyper::{header, Body, Request, Response};
use hyper::service::service_fn;

const PHRASE: &'static str = "It's a Unix system. I know this.";

fn hello(_: Request<Body>) -> impl futures::Future<Item = Response<Body>, Error = io::Error> + Send {
    futures::future::ok(
        Response::builder()
            .header(header::CONTENT_TYPE, "text/plain")
            .header(header::CONTENT_LENGTH, PHRASE.len() as u64)
            .body(PHRASE.into())
            .expect("failed to create response")
    )
}

fn run() -> io::Result<()> {
    let path = "test.sock";
    if let Err(err) = fs::remove_file("test.sock") {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err);
        }
    }
    let svr = hyperlocal_windows::server::Server::new().bind(path, || service_fn(hello))?;
    println!("Listening on unix://{path} with 1 thread.", path = path);
    svr.run()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error starting server: {}", err)
    }
}
```

### clients

You can communicate over HTTP with Unix domain socket servers using hyper's Client interface.
Configure your hyper client using `Client::configure(...)`.

Hyper's client
interface makes it easy to issue typical HTTP methods like GET, POST, DELETE with factory methods,
`get`, `post`, `delete`, ect. These require an argument that can be tranformed into a `hyper::Uri`.
Since unix domain sockets aren't represented with hostnames that resolve to ip addresses coupled with network ports ports,
your standard url string won't do. Instead, use a `hyperlocal_windows::Uri`
which represents both file path to the domain socket and the resource uri path and query string.

```rust
extern crate futures;
extern crate hyper;
extern crate hyperlocal_windows;
extern crate tokio_core;

use std::io::{self, Write};

use futures::Stream;
use futures::Future;
use hyper::{Client, rt};
use hyperlocal_windows::{Uri, UnixConnector};

fn main() {
    let client = Client::builder()
        .keep_alive(false) // without this the connection will remain open
        .build::<_, ::hyper::Body>(UnixConnector::new());
    let url = Uri::new("test.sock", "/").into();

    let work = client
        .get(url)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        });

    rt::run(work);
}
```

# Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.microsoft.com.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.
