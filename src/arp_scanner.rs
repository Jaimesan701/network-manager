extern crate pnet;
extern crate pnet_datalink;


use std::thread;

use crate::network_user::NetworkUser;
use crate::packet_factory::PacketFactory;
use crate::packet_initializer::PacketInitializer;

use pnet::packet::Packet;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::EthernetPacket;
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::DataLinkReceiver;
use pnet_datalink::NetworkInterface;
use pnet_datalink::interfaces;
use pnet_datalink::channel;

pub struct ArpScanner{

    scanner_retries : u8,
    interface : NetworkInterface

}

impl ArpScanner{

    
    pub fn new(interface_name : &str) -> Option<ArpScanner>{
             
        let scanner = match interfaces().iter().find(|i| i.is_up() && i.name.eq_ignore_ascii_case(interface_name)) {

            Some(i) => {

                ArpScanner{
                    interface : i.clone(),
                    scanner_retries : 5
                }
            },

            None => panic!("Interface not found"),

        };

        Some(scanner)

    }

    fn send_packets(){

    }

    fn rcv_packets(){

    }

    pub fn perform_scanner(&self){

            let mut users : Vec<NetworkUser> = vec![];
            let channel = channel(&self.interface, Default::default());


            let (mut tx, mut rx) = match channel {
                Ok(Ethernet(tx,rx)) => (tx,rx),
                Ok(_) => panic!("Failed"),
                Err(e) => panic!("Failed, {e}")
            };

            let network_ip = self.interface.ips[0];

            let my_ip = network_ip.ip();
            let my_mac = self.interface.mac.unwrap();

            for (i , target_ip) in network_ip.iter().enumerate(){
                
                println!("Ip actual: {target_ip}");

                let mut arp_packet = PacketFactory::create_arp_packet();
                PacketInitializer::initialize_arp_request_packet(&mut arp_packet, my_mac, my_ip, target_ip);

                let mut ethernet_packet = PacketFactory::create_ethernet_packet();
                PacketInitializer::initialize_ethernet_packet(&mut ethernet_packet, my_mac, arp_packet.packet());
                
                for j in 1..self.scanner_retries{

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
                            
                            let user = NetworkUser{
                                ip: arp_packet.get_target_proto_addr(),
                                mac: arp_packet.get_target_hw_addr(),
                            };
                            
                            users.push(user);
                        }
            
                    },
                    Err(_) => println!("No ha sido posible recibir una trama"),
                
                }


            }

            //println!("{:#?}",users);
        }

    }
}