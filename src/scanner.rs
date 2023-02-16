extern crate pnet;

use std::collections::HashMap;
use std::sync::mpsc::{self, Sender, Receiver};
use std::{thread, time};
use std::time::Duration;

use crate::manager_factory::{ManagerType, ManagerFactory};
use crate::network_user::NetworkUser;
use crate::scanner_manager::{ScannerManager};

use pnet::datalink::{NetworkInterface,interfaces,channel, DataLinkSender, DataLinkReceiver};
use pnet::ipnetwork::IpNetwork;
use pnet::packet::Packet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::datalink::Channel::Ethernet;
use pnet::util::MacAddr;


pub struct Scanner{

    scanner_retries : u8,
    interface : NetworkInterface,
    manager_type: ManagerType

}

impl Scanner{

    
    pub fn new(interface_name : &str, manager_type : ManagerType) -> Option<Scanner>{
             
        let scanner = match interfaces().iter().find(|i| i.is_up() && i.name.eq_ignore_ascii_case(interface_name)) {

            Some(i) => {

                Scanner{
                    
                    interface : i.clone(),
                    scanner_retries : 5,
                    manager_type
                }
            },

            None => panic!("Interface not found"),

        };

        Some(scanner)

    }



    
    pub fn perform_scanner(&self){

            let mut users : HashMap<MacAddr,NetworkUser> = HashMap::new();

            let channel = channel(&self.interface, Default::default());

            let (tx, rx) = match channel {
                Ok(Ethernet(tx,rx)) => (tx,rx),
                Ok(_) => panic!("Failed"),
                Err(e) => panic!("Failed, {e}")
            };
            
            let network_ip = self.interface.ips[0];
            let my_mac = self.interface.mac.unwrap();

            let manager = ManagerFactory::new_manager(&self.manager_type) as Box<dyn ScannerManager + Send>; 
            let (producer_tx, consumer_tx) = mpsc::channel::<Option<NetworkUser>>();
            let handler_rx = thread::spawn(|| Scanner::receive_packet(rx,producer_tx,manager));

            let manager = ManagerFactory::new_manager(&self.manager_type) as Box<dyn ScannerManager + Send>;   
            let handler_tx = thread::spawn(move || Scanner::send_packet(tx,manager,network_ip,my_mac));


            
            for data_rcv in consumer_tx{

                match data_rcv{

                    Some(user) => users.insert(user.mac, user),
                    None => None,

                };

            }

            handler_tx.join();
            handler_rx.join();
            
            println!("{:#?}",users);

        }


        
        fn send_packet(mut transmitter : Box<dyn DataLinkSender>, scanner_manager : Box<dyn ScannerManager>, ip: IpNetwork, my_mac: MacAddr){

            let my_ip = ip.ip();

            for (_i,target_ip) in ip.iter().enumerate(){

                for _i in 1..10{
                
                    let packet = scanner_manager.create_packet(my_ip,target_ip,my_mac);
                    match transmitter.send_to(&packet[0..], None){
                    
                        Some(r) => match r {
                            Ok(_) => {},
                            Err(e) => panic!("{e}"),
                        },
                        None => println!("Packet hasn't been sended"),
                    };
            
                }


            }
                
    
        }
    
        fn receive_packet(mut receiver: Box<dyn DataLinkReceiver>,producer_tx : Sender<Option<NetworkUser>>,scanner_manager : Box<dyn ScannerManager>){
            
            let mut rcv_termination = false;
            while !rcv_termination{
    
                match receiver.next(){
    
                    Ok(packet) => {
                        
                        let ethernet_packet = EthernetPacket::new(packet).unwrap_or_else(|| panic!("No se ha podido decodificar el paquete"));
                        let user = scanner_manager.parse_packet(ethernet_packet); 
                        
                        producer_tx.send(user);
            
                    },
                    Err(_) => println!("No ha sido posible recibir una trama"),
                
                };
    
            }
    
        }

    }



