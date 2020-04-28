use std::fmt::Write;

use tiny_http::{Request, Response, Server};

use crate::constants::*;
use crate::my_err::MyErr;
use crate::utils::is_debug;

pub mod constants;
pub mod my_err;
pub mod utils;

fn main() -> Result<(), MyErr> {
  let server = Server::http(default_ipv4_sockaddr()).map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))?;

  println!("Listening on [{}]!", server.server_addr());

  for req in server.incoming_requests() {
    if is_debug() {
      println!("[{:^7}] @ [{}]", req.method().as_str(), req.url());
    }
    handle(req)?;
  }

  Ok(())
}

#[inline]
fn handle(req: Request) -> Result<(), MyErr> {
  let url = req.url();

  // filter requests that do not come from [/api/{API_VERSION}]
  if url.len() < API_PREFIX.len() + API_VERSION.len()
    || !url.starts_with(API_PREFIX)
    || !(&url[API_PREFIX.len()..]).starts_with(API_VERSION) {
    req.respond(Response::empty(403)).map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1))?;
    return Ok(());
  }

  let mut resp_str = String::new();

  write!(&mut resp_str, "[{:^7}] @ [{}]", req.method().as_str(), req.url()).map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))?;

  for h in req.headers() {
    write!(&mut resp_str, "\n{:?}", h).map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1))?;
  }

  req.respond(Response::from_string(resp_str)).map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))?;

  Ok(())
}