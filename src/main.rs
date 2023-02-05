use manager_factory::ManagerType;
use scanner::Scanner;

pub mod packet_initializer;
pub mod packet_factory;
pub mod scanner;
pub mod scanner_manager;
pub mod network_user;
pub mod manager_factory;

extern crate pnet;



fn main() {
  

   let scanner = Scanner::new("ens33",ManagerType::ARP).unwrap_or_else(|| panic!("Error with the interface"));
   scanner.perform_scanner();

  
     
      /*   
      let mut arp_packet =  packet_factory.create_arp_packet();
     
      packet_init.initialize_arp_request_packet(&mut arp_packet, my_mac, Ipv4Addr::new(192, 168, 1, 40), Ipv4Addr::new(192, 168, 1, 36));

      let mut ethernet_packet = packet_factory.create_ethernet_packet();

      packet_init.initialize_ethernet_packet(&mut ethernet_packet, my_mac,arp_packet.packet());
    
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
       */

       

   

}
