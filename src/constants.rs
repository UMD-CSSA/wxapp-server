#![allow(dead_code)]

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4};

// Default listening address
pub(crate) const DEFAULT_PORT: u16 = 6006;

pub(crate) const DEFAULT_IPV4_ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;
pub(crate) const DEFAULT_IPV6_ADDR: Ipv6Addr = Ipv6Addr::LOCALHOST;

#[inline]
pub(crate) fn default_ipv4_sockaddr() -> SocketAddrV4 {
  SocketAddrV4::new(DEFAULT_IPV4_ADDR, DEFAULT_PORT)
}

// api version
pub(crate) const API_VERSION: &str = "/v0/";
pub(crate) const MINIAPP_LOGIN_URL: &str = "miniapp/login";