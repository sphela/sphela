use crate::configure;

use std::fs::File;
use std::io::prelude::*;

use std::sync::{Arc, RwLock};

use hyper::{Body, Response, Server, StatusCode};
use hyper::header::{HeaderValue};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

pub fn run (config: &configure::Config) {
    let doc_root = config.server.doc_root.clone();
    let addr = config.server.addr.clone();

    let index_path = format!("{}/{}", doc_root, "index.html");
    let mut f = File::open(&index_path).expect("index.html file not found");
    let mut html_contents = String::new();
    f.read_to_string(&mut html_contents)
        .expect(&format!("something went wrong reading the {} file", index_path));

    let html_contents = Arc::new(RwLock::new(html_contents));
    let new_service = move || {
        let html_c = html_contents.clone();
            service_fn_ok( move |_| {
                let mut response: Response<Body> = Response::default();
                match html_c.try_read() {
                    Ok(c) => {
                        *response.status_mut() = StatusCode::OK;
                        response.headers_mut().insert("X-Thank-You", HeaderValue::from_static("For using Sphela!"));
                        *response.body_mut() = Body::from(format!("{}", c));
                    }
                    Err(_) => {
                        println!("Unable to read the html file for serving!");
                        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    },
                };
                response
        })
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
