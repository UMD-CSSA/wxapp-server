#![feature(maybe_uninit_extra)]
#![feature(thread_spawn_unchecked)]

use std::fmt::Write;
use std::mem::MaybeUninit;
use std::thread;

use tiny_http::{Header, Server};

use crate::constants::*;
use crate::my_err::MyErr;
use crate::utils::{code_response, str_response};

pub mod constants;
pub mod my_err;
pub mod utils;

static mut CONTENT_TYPE_JSON_HEADER: MaybeUninit<Header> =
  MaybeUninit::uninit();
pub(crate) static mut DATE_HEADER_EMPTY: MaybeUninit<Header> =
  MaybeUninit::uninit();
pub(crate) static mut SERVER_HEADER_EMPTY: MaybeUninit<Header> =
  MaybeUninit::uninit();

fn main() -> Result<(), MyErr> {
  // cache variables
  {
    // content-type:json
    unsafe {
      let key = "Content-Type";
      CONTENT_TYPE_JSON_HEADER.write(Header::from_bytes(key, "application/json")
        .map_err(|err| MyErr::from_str(format!(
          "Failed to generate the header [{}]! Err: {:?}", key, err
        ), file!(), line!() - 3))?);
    }

    // date
    unsafe {
      let key = "Date";
      DATE_HEADER_EMPTY.write(Header::from_bytes(key, "")
        .map_err(|err| MyErr::from_str(format!(
          "Failed to generate the header [{}]! Err: {:?}", key, err
        ), file!(), line!() - 3))?);
    }

    // server
    unsafe {
      let key = "Server";
      SERVER_HEADER_EMPTY.write(Header::from_bytes(key, "")
        .map_err(|err| MyErr::from_str(format!(
          "Failed to generate the header [{}]! Err: {:?}", key, err
        ), file!(), line!() - 3))?);
    }
  }

  let server = Server::http(default_ipv4_sockaddr()).map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))?;

  println!("Listening on [{}]!", server.server_addr());

  for req in server.incoming_requests() {
    let url = req.url();
    let remote_addr =req.remote_addr();
    println!("[{:^7}] @ [{}] from {}", req.method().as_str(), url, remote_addr);

    // filter requests that do not come from [/api/{API_VERSION}]
    if url.len() < API_VERSION.len() || !url.starts_with(API_VERSION) {
      return req.respond(code_response(403)).map_err(
        |err| MyErr::from_err(&err, file!(), line!() - 1));
    }

    // handle real request
    unsafe {
      thread::Builder::new()
        .name(remote_addr.to_string())
        .stack_size(1024)
        .spawn(move || {
          {
            let url = req.url();
            if url[API_VERSION.len()..].starts_with(MINIAPP_LOGIN_URL) {
              // handle wxapp login
              {
                let ptn = "?code=";
                let param = &req.url()[API_VERSION.len() + MINIAPP_LOGIN_URL.len()..];

                if !param.starts_with(ptn) {
                  return req.respond(code_response(400)).map_err(
                    |err| MyErr::from_err(&err, file!(), line!() - 1));
                }

                // request wxapp server
                let _3rd_session = "";//todo
                {
                  let code = &param[ptn.len()..];
                  println!("wxapp code: {}", code);
                  // let response = minreq::get(
                  //   "https://api.weixin.qq.com/sns/jscode2session?appid=APPID&secret=SECRET&js_code=JSCODE&grant_type=authorization_code"
                  // ).send()?;
                }

                let resp =
                  str_response(format!(
                    "{{\"_3rd_session\":\"{}\"}}", _3rd_session))
                    .with_header(CONTENT_TYPE_JSON_HEADER.read());

                req.respond(resp).map_err(
                  |err| MyErr::from_err(&err, file!(), line!() - 1))
              }
            } else {
              // just echo
              {
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
            }
          }
        })
        .map_err(|err| MyErr::from_err(&err, file!(), line!()))?;
    }

    println!("Ready for new connection!");
  }

  Ok(())
}