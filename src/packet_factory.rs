
extern crate pnet;




use pnet::packet::arp::{MutableArpPacket };
use pnet::packet::ethernet::{MutableEthernetPacket};
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::ipv4::MutableIpv4Packet;

pub struct PacketFactory{


}


impl PacketFactory{


    pub fn create_ethernet_packet() -> MutableEthernetPacket<'static>{

        let buffer:Vec<u8> = vec![0;1500];
        MutableEthernetPacket::owned(buffer).unwrap_or_else(|| panic!("No se pudo crear un paquete tipo ethernet"))

    }

    pub fn create_arp_packet() -> MutableArpPacket<'static>{

        let buffer:Vec<u8> = vec![0;28];
        MutableArpPacket::owned(buffer).unwrap_or_else(|| panic!("No se pudo crear un paquete tipo arp"))

    }

    pub fn create_ip_packet() -> MutableIpv4Packet<'static>{

        let buffer:Vec<u8> = vec![0;576];
        MutableIpv4Packet::owned(buffer).unwrap_or_else(|| panic!("No se pudo crear un paquete tipo ip"))

    }

    pub fn create_echo_request_packet() -> MutableEchoRequestPacket<'static>{

        let buffer:Vec<u8> = vec![0;32];
        MutableEchoRequestPacket::owned(buffer).unwrap_or_else(|| panic!("No se pudo crear un paquete tipo request"))

    }


}