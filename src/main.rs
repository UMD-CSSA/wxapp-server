use crate::constants::*;
use crate::my_err::MyErr;

pub mod constants;
pub mod my_err;
pub mod utils;

fn main() -> Result<(), MyErr> {
  use tiny_http::{Server, Response};

  let server = Server::http(default_ipv4_sockaddr()).map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))?;

  println!("Listening on [{}]!", server.server_addr());

  for request in server.incoming_requests() {
    // println!("received request! method: {:?}, url: {:?}, headers: {:?}",
    //          request.method(),
    //          request.url(),
    //          request.headers()
    // );
    println!("[{:^7}] @ [{}]",
             request.method().as_str(),
             request.url()
    );

    let response = Response::from_string(
      format!("[{:^7}] @ [{}]\n{:?}",
              request.method().as_str(), request.url(),request.headers()));

    request.respond(response).map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1))?;
  }

  Ok(())
}