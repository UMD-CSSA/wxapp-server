#![allow(dead_code)]

use std::io::{Cursor, Empty};

use tiny_http::Response;

use crate::{DATE_HEADER_EMPTY, SERVER_HEADER_EMPTY};

#[inline]
pub(crate) const fn is_debug() -> bool {
  cfg!(debug_assertions)
}

#[inline]
pub(crate) fn str_response<T: Into<String>>(str: T) -> Response<Cursor<Vec<u8>>> {
   Response::from_string(str)
    .with_header(unsafe { DATE_HEADER_EMPTY.get_ref() }.clone())
    .with_header(unsafe { SERVER_HEADER_EMPTY.get_ref() }.clone())
}

#[inline]
pub(crate) fn code_response(code: u16) -> Response<Empty> {
  Response::empty(code)
    .with_header(unsafe { DATE_HEADER_EMPTY.get_ref() }.clone())
    .with_header(unsafe { SERVER_HEADER_EMPTY.get_ref() }.clone())
}