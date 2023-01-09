extern crate pnet;
extern crate pnet_datalink;


use  std::net::Ipv4Addr;

use pnet::packet::{MutablePacket, Packet};
use pnet::packet::arp::{MutableArpPacket,ArpOperations, ArpPacket};
use pnet::packet::ethernet::{MutableEthernetPacket,EtherTypes, EthernetPacket};
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::MacAddr;
use pnet_datalink::interfaces;
use pnet_datalink::channel;

fn initialize_arp_request_packet(arp_packet : &mut MutableArpPacket, my_mac : MacAddr, my_ip : Ipv4Addr, target_ip : Ipv4Addr){

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

fn initialize_ethernet_packet(ethernet_packet : &mut MutableEthernetPacket, my_mac : MacAddr){

   ethernet_packet.set_destination(MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff));
   ethernet_packet.set_source(my_mac);
   ethernet_packet.set_ethertype(EtherTypes::Arp);

}

fn main() {
  
   let interfaces = interfaces();
   let my_interface = interfaces.get(1).unwrap_or_else(|| panic!("No se puede obtener la interfaz"));
   println!("{:#?}",my_interface);
   let my_mac = my_interface.mac.unwrap_or_else(|| panic!("No se puede obtener la MAC"));
   let channel = channel(my_interface, Default::default());

   

   let (mut tx, mut rx) = match channel {
      
      Ok(Ethernet(tx,rx)) => (tx,rx),
      Ok(_) => panic!("Failed"),
      Err(e) => panic!("Failed, {e}")

   };


   loop {
    
      let buffer:Vec<u8> = vec![0;30];
      let mut arp_packet = match MutableArpPacket::owned(buffer){
         Some(p) => p,
         None => panic!("Error creando mensaje ARP")
      };
     
      initialize_arp_request_packet(&mut arp_packet, my_mac, Ipv4Addr::new(192, 168, 1, 134), Ipv4Addr::new(192, 168, 1, 36));

      let mut buffer = [0u8; 100];
      let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer).unwrap_or_else(|| panic!("No se pudo crear un paquete tipo ethernet"));

      initialize_ethernet_packet(&mut ethernet_packet, my_mac);
      ethernet_packet.set_payload(arp_packet.packet_mut());
      
      let packet = ethernet_packet.packet();
      let packet_len = packet.len(); 
      println!("{packet_len}");
      
      match tx.send_to(ethernet_packet.packet(), None){

        Some(r) => match r {
            Ok(_) => {},
            Err(e) => panic!("{e}"),
        },
        None => println!("Paquete NO enviado"),

      };

      match rx.next(){

        Ok(packet) => {

            let ethernet_packet = EthernetPacket::new(packet).unwrap_or_else(|| panic!("No se ha podido decodificar el paquete"));

            if (ethernet_packet.get_ethertype() == EtherTypes::Arp){

               let arp_packet = ArpPacket::new(ethernet_packet.payload()).unwrap_or_else(|| panic!("No se ha podido decodificar el paquete"));  
               println!("{:#?}",arp_packet);

            }

        },
        Err(_) => println!("No ha sido posible recibir una trama"),
      
      }
       

   }

}
