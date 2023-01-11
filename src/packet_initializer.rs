extern crate pnet;
extern crate pnet_datalink;


use std::net::Ipv4Addr;
use pnet::packet::arp::{MutableArpPacket,ArpOperations };
use pnet::packet::ethernet::{MutableEthernetPacket,EtherTypes};
use pnet_datalink::MacAddr;


pub struct PacketInitializer{

}

impl PacketInitializer {

    pub fn initialize_arp_request_packet(&self,arp_packet : &mut MutableArpPacket, my_mac : MacAddr, my_ip : Ipv4Addr, target_ip : Ipv4Addr){

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
     
    pub fn initialize_ethernet_packet(&self,ethernet_packet : &mut MutableEthernetPacket, my_mac : MacAddr){
     
        ethernet_packet.set_destination(MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff));
        ethernet_packet.set_source(my_mac);
        ethernet_packet.set_ethertype(EtherTypes::Arp);
     
     }

}