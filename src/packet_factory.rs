
extern crate pnet;



use pnet::packet::arp::{MutableArpPacket };
use pnet::packet::ethernet::{MutableEthernetPacket};

pub struct PacketFactory{


}


impl PacketFactory{


    pub fn create_ethernet_packet() -> MutableEthernetPacket<'static>{
        
        let buffer:Vec<u8> = vec![0;1500];
        MutableEthernetPacket::owned(buffer).unwrap_or_else(|| panic!("No se pudo crear un paquete tipo ethernet"))
    
    

    }

    pub fn create_arp_packet() -> MutableArpPacket<'static>{

        let buffer:Vec<u8> = vec![0;28];
        MutableArpPacket::owned(buffer).unwrap_or_else(|| panic!("No se pudo crear un paquete tipo ethernet"))

    
    }

}