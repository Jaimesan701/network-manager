extern crate pnet;
extern crate pnet_datalink;


use std::collections::HashMap;
use std::thread;

use crate::network_user::NetworkUser;
use crate::packet_factory::PacketFactory;
use crate::packet_initializer::PacketInitializer;

use pnet::packet::Packet;
use pnet::packet::arp::ArpOperations;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::EthernetPacket;
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::MacAddr;
use pnet_datalink::NetworkInterface;
use pnet_datalink::interfaces;
use pnet_datalink::channel;

pub struct IcmpScanner{

    scanner_retries : usize,
    interface : NetworkInterface

}


impl IcmpScanner{

    pub fn new(interface_name : &str) -> Option<IcmpScanner>{
                
        let scanner = match interfaces().iter().find(|i| i.is_up() && i.name.eq_ignore_ascii_case(interface_name)) {

            Some(i) => {

                IcmpScanner{
                    interface : i.clone(),
                    scanner_retries : 5
                }
            },

            None => panic!("Interface not found"),

        };

        Some(scanner)

    }

    pub fn perform_scanner(&self){

            let mut users : HashMap<MacAddr,NetworkUser> = HashMap::new();

            let channel = channel(&self.interface, Default::default());

            let (mut tx, mut rx) = match channel {
                Ok(Ethernet(tx,rx)) => (tx,rx),
                Ok(_) => panic!("Failed"),
                Err(e) => panic!("Failed, {e}")
            };
            
            let network_ip = self.interface.ips[0];

            let my_ip = network_ip.ip();
            let my_mac = self.interface.mac.unwrap();
        
            let handler_tx = thread::spawn(move ||{
                       
            for (i , target_ip) in network_ip.iter().enumerate(){
            
                println!("Ip actual: {target_ip}");

                let mut echo_request_packet = PacketFactory::create_echo_request_packet();
                PacketInitializer::initialize_echo_request_packet(&mut echo_request_packet);

                let mut ip_packet = PacketFactory::create_ip_packet();
                PacketInitializer::initialize_ip_packet(&mut ip_packet,my_ip,target_ip,echo_request_packet.packet());

                let mut ethernet_packet = PacketFactory::create_ethernet_packet();
                let target_mac = MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
                PacketInitializer::initialize_ethernet_packet(&mut ethernet_packet, my_mac,target_mac,EtherTypes::Ipv4, ip_packet.packet());
                
                for j in 1..20{

                    
                   
                    match tx.send_to(ethernet_packet.packet(), None){
    
                        Some(r) => match r {
                            Ok(_) => {},
                            Err(e) => panic!("{e}"),
                        },
                        None => println!("Paquete NO enviado"),
                    };
                    

                }
                    
            };

        });

        println!("scanner finished");
        handler_tx.join();
    }

}