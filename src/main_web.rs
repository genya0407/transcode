extern crate hyper;
extern crate transcode;
extern crate futures;

use hyper::{Body, Request, Response, Server};
use hyper::service::service_fn;
use hyper::rt::{self, Future};
use futures::Stream;

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| {
            service_fn(move |req: Request<Body>| {
                req.into_body().concat2().map(|body| {
                    match String::from_utf8(body.to_vec()) {
                        Ok(body) => {
                            match transcode::parser::parse_json(body) {
                                Ok(context) => {
                                    let mut evaluator = transcode::evaluator::Evaluator { context: context };
                                    evaluator.run();
                                    let response_json_string = transcode::printer::print(evaluator.context);
                                    Response::new(Body::from(response_json_string))
                                },
                                _ => Response::new(Body::from(""))
                            }
                        }
                        _ => Response::new(Body::from(""))
                    }
                })
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}