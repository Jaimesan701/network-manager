
use std::net::Ipv4Addr;
use pnet_datalink::MacAddr;

#[derive(Debug)]
pub struct NetworkUser{

    pub ip : Ipv4Addr,
    pub mac : MacAddr

}