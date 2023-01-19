extern crate pnet;
extern crate pnet_datalink;


use std::net::{ IpAddr };
use pnet::packet::Packet;
use pnet::packet::arp::{MutableArpPacket,ArpOperations };
use pnet::packet::ethernet::{MutableEthernetPacket, EtherType};
use pnet::packet::icmp::{IcmpTypes, IcmpPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{MutableIpv4Packet, checksum, Ipv4Packet};
use pnet_datalink::MacAddr;
use pnet::packet::icmp::echo_request::{IcmpCodes, MutableEchoRequestPacket};

pub struct PacketInitializer{

}

impl PacketInitializer {

    pub fn initialize_arp_request_packet(arp_packet : &mut MutableArpPacket, my_mac : MacAddr, my_ip : IpAddr, target_ip : IpAddr){
       
        let my_ip = match my_ip{
            std::net::IpAddr::V4(ip) => ip,
            std::net::IpAddr::V6(ip) => panic!("ARP scanner cannot be performed with IPv6 addresses"),
        };

        let target_ip = match target_ip{
            std::net::IpAddr::V4(ip) => ip,
            std::net::IpAddr::V6(ip) => panic!("ARP scanner cannot be performed with IPv6 addresses"),
        };

        //Hardware type for ethernet is 1
        arp_packet.set_hardware_type(pnet::packet::arp::ArpHardwareType::new(1));
        //Value for ethernet
        arp_packet.set_protocol_type(pnet::packet::ethernet::EtherType::new(0x0800));
        
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        
        //Value for request is 1
        arp_packet.set_operation(ArpOperations::Request);
     
        arp_packet.set_sender_hw_addr(my_mac);
        arp_packet.set_sender_proto_addr(my_ip);
     
        arp_packet.set_target_hw_addr(MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff));
        arp_packet.set_target_proto_addr(target_ip);
     
     }
     
    pub fn initialize_ethernet_packet(ethernet_packet : &mut MutableEthernetPacket,my_mac : MacAddr, target_mac : MacAddr, ether_type : EtherType, payload : &[u8]){
     
        ethernet_packet.set_destination(target_mac);
        ethernet_packet.set_source(my_mac);
        ethernet_packet.set_ethertype(ether_type);
        ethernet_packet.set_payload(payload);

     }


    pub fn initialize_ip_packet(ip_packet : &mut MutableIpv4Packet, my_ip : IpAddr, target_ip : IpAddr, payload : &[u8]){

        let my_ip = match my_ip{
            std::net::IpAddr::V4(ip) => ip,
            std::net::IpAddr::V6(ip) => panic!("ARP scanner cannot be performed with IPv6 addresses"),
        };

        let target_ip = match target_ip{
            std::net::IpAddr::V4(ip) => ip,
            std::net::IpAddr::V6(ip) => panic!("ARP scanner cannot be performed with IPv6 addresses"),
        };

        ip_packet.set_version(4);
        ip_packet.set_header_length(5);
        ip_packet.set_dscp(0);
        ip_packet.set_ecn(0);
        ip_packet.set_total_length((ip_packet.packet().len()+payload.len()).try_into().unwrap());
        let id = rand::random::<u16>();
        ip_packet.set_identification(id);
        ip_packet.set_flags(0);
        ip_packet.set_fragment_offset(0);
        ip_packet.set_ttl(64);
        ip_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
        ip_packet.set_source(my_ip);
        ip_packet.set_destination(target_ip);
        ip_packet.set_payload(payload);
        
        ip_packet.set_checksum(checksum(&Ipv4Packet::new(ip_packet.packet()).unwrap()));
        
    
     }

    pub fn initialize_echo_request_packet(echo_request_packet : &mut MutableEchoRequestPacket){

        echo_request_packet.set_icmp_type(IcmpTypes::EchoRequest);
        echo_request_packet.set_icmp_code(IcmpCodes::NoCode);
        let id = rand::random::<u16>();
        echo_request_packet.set_identifier(id);
        echo_request_packet.set_payload("dsofidsfds".as_bytes());
        echo_request_packet.set_checksum(pnet::packet::icmp::checksum(&IcmpPacket::new(echo_request_packet.packet()).unwrap()));
       
    }


}