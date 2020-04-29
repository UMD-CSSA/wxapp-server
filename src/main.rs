#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_ref)]
#![feature(thread_id_value)]

use std::cmp::max;
use std::fmt::Write;
use std::mem::MaybeUninit;
use std::thread;

use threadpool::ThreadPool;
use tiny_http::{Header, Request, Server};

use crate::constants::*;
use crate::my_err::MyErr;
use crate::utils::{code_response, str_response};

pub mod constants;
pub mod my_err;
pub mod utils;

static mut CONTENT_TYPE_JSON_HEADER: MaybeUninit<Header> = MaybeUninit::uninit();
pub(crate) static mut DATE_HEADER_EMPTY: MaybeUninit<Header> = MaybeUninit::uninit();
pub(crate) static mut SERVER_HEADER_EMPTY: MaybeUninit<Header> = MaybeUninit::uninit();

fn main() -> Result<(), MyErr> {
  // cache variables
  {
    // content-type:json
    {
      let key = "Content-Type";
      unsafe {
        CONTENT_TYPE_JSON_HEADER.write(Header::from_bytes(key, "application/json")
          .map_err(|err| MyErr::from_str(format!(
            "Failed to generate the header [{}]! Err: {:?}", key, err
          ), file!(), line!() - 3))?);
      }
    }

    // date
    {
      let key = "Date";
      unsafe {
        DATE_HEADER_EMPTY.write(Header::from_bytes(key, "")
          .map_err(|err| MyErr::from_str(format!(
            "Failed to generate the header [{}]! Err: {:?}", key, err
          ), file!(), line!() - 3))?);
      }
    }

    // server
    {
      let key = "Server";
      unsafe {
        SERVER_HEADER_EMPTY.write(Header::from_bytes(key, "")
          .map_err(|err| MyErr::from_str(format!(
            "Failed to generate the header [{}]! Err: {:?}", key, err
          ), file!(), line!() - 3))?);
      }
    }
  }

  // init threadpool
  let threadpool;
  {
    threadpool = ThreadPool::new(
      max(4, (num_cpus::get() as f32 * 1.5) as usize));
    println!("Threadpool built with [{}/{}] threads!",
             threadpool.active_count(), threadpool.max_count());
  }

  // start server
  let server;
  {
    server = Server::http(default_ipv4_sockaddr()).map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1))?;

    println!("Listening on [{}]!", server.server_addr());
  }

  for req in server.incoming_requests() {
    let url = req.url();
    let remote_addr = req.remote_addr();
    println!("[{:^7}] from {} @ [{}]", req.method().as_str(), remote_addr, url);

    // filter requests that do not come from [/{API_VERSION}]
    if url.len() < API_VERSION.len() || !url.starts_with(API_VERSION) {
      req.respond(code_response(403)).map_err(
        |err| MyErr::from_err(&err, file!(), line!() - 1))?;
      continue;
    }

    // handle real request
    threadpool.execute(|| handle_request(req).unwrap());
  }

  Ok(())
}

#[inline]
fn handle_request(req: Request) -> Result<(), MyErr> {
  println!("Request from [{}] has been moved to thread [{}]!",
           thread::current().name().unwrap_or_default(),
           thread::current().id().as_u64());

  if req.url()[API_VERSION.len()..].starts_with(MINIAPP_LOGIN_URL) {
    // handle wxapp login
    handle_wxapp_login(req)
  } else {
    // just echo
    handle_echo(req)
  }
}

#[inline]
fn handle_wxapp_login(req: Request) -> Result<(), MyErr> {
  let ptn = "?code=";
  let param = &req.url()[API_VERSION.len() + MINIAPP_LOGIN_URL.len()..];

  if !param.starts_with(ptn) {
    return req.respond(code_response(400)).map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1));
  }

  // request wxapp server
  let _3rd_session;//todo
  {
    let code = &param[ptn.len()..];
    println!("wxapp code: {}", code);
    _3rd_session = format!("Your code was: {}, this API is still under construction!", code);

    // let response = minreq::get(
    //   "https://api.weixin.qq.com/sns/jscode2session?appid=APPID&secret=SECRET&js_code=JSCODE&grant_type=authorization_code"
    // ).send()?;
  }

  let resp =
    str_response(format!(
      "{{\"_3rd_session\":\"{}\"}}", _3rd_session))
      .with_header(unsafe { CONTENT_TYPE_JSON_HEADER.get_ref() }.clone());

  req.respond(resp).map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))
}

#[inline]
fn handle_echo(req: Request) -> Result<(), MyErr> {
  let mut resp_str;

  // write echo
  {
    resp_str = format!("[{:^7}] @ [{}]", req.method().as_str(), req.url());

    for h in req.headers() {
      write!(&mut resp_str, "\n{:?}", h).map_err(
        |err| MyErr::from_err(&err, file!(), line!() - 1))?;
    }
  }

  req.respond(str_response(resp_str)).map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))
}