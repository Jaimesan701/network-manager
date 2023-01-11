pub mod packet_initializer;


extern crate pnet;
extern crate pnet_datalink;

use packet_initializer::PacketInitializer;


use std::net::Ipv4Addr;

use pnet::packet::{MutablePacket, Packet};
use pnet::packet::arp::{MutableArpPacket, ArpPacket};
use pnet::packet::ethernet::{MutableEthernetPacket,EtherTypes, EthernetPacket};
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::interfaces;
use pnet_datalink::channel;

fn main() {
  
   let interfaces = interfaces();
   let my_interface = interfaces.get(2).unwrap_or_else(|| panic!("No se puede obtener la interfaz"));
   println!("{:#?}",my_interface);
   let my_mac = my_interface.mac.unwrap_or_else(|| panic!("No se puede obtener la MAC"));
   let channel = channel(my_interface, Default::default());

   let packet_init = PacketInitializer{
   
   };

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
     
      packet_init.initialize_arp_request_packet(&mut arp_packet, my_mac, Ipv4Addr::new(192, 168, 1, 40), Ipv4Addr::new(192, 168, 1, 36));

      let mut buffer = [0u8; 100];
      let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer).unwrap_or_else(|| panic!("No se pudo crear un paquete tipo ethernet"));

      packet_init.initialize_ethernet_packet(&mut ethernet_packet, my_mac);
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

            if ethernet_packet.get_ethertype() == EtherTypes::Arp{

               let arp_packet = ArpPacket::new(ethernet_packet.payload()).unwrap_or_else(|| panic!("No se ha podido decodificar el paquete"));  
               println!("{:#?}",arp_packet);

            }

        },
        Err(_) => println!("No ha sido posible recibir una trama"),
      
      }
       

   }

}
