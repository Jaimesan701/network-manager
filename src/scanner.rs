extern crate pnet;

use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::manager_factory::ManagerType;
use crate::network_user::NetworkUser;
use crate::scanner_manager::ScannerManager;

use pnet::datalink::{NetworkInterface,interfaces,channel, DataLinkSender, DataLinkReceiver};
use pnet::packet::ethernet::EthernetPacket;
use pnet::util::MacAddr;
use pnet::datalink::Channel::Ethernet;


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

            let (mut tx, mut rx) = match channel {
                Ok(Ethernet(tx,rx)) => (tx,rx),
                Ok(_) => panic!("Failed"),
                Err(e) => panic!("Failed, {e}")
            };
            
            let network_ip = self.interface.ips[0];
            let my_ip = network_ip.ip();
            let my_mac = self.interface.mac.unwrap();


            let handler_tx = thread::spawn(|| Scanner::send_packet(tx));
        
            let (producer_tx, consumer_tx) = mpsc::channel::<NetworkUser>();
            let handler_rx = thread::spawn(|| Scanner::receive_packet(rx,producer_tx));
            
            for data_rcv in consumer_tx{

                users.insert(data_rcv.mac, data_rcv);
            
            }

            handler_tx.join();
            handler_rx.join();
            
            
            println!("{:#?}",users);

        }


        
        fn send_packet(transmitter : Box<dyn DataLinkSender>, scanner_manager : Box<dyn ScannerManager>){

            for i in 1..20{
                
                let packet = scanner_manager.create_packet();
                match transmitter.send_to(packet, None){
                
                    Some(r) => match r {
                        Ok(_) => {},
                        Err(e) => panic!("{e}"),
                    },
                    None => println!("Packet hasn't been sended"),
                };
           
            }
    
        }
    
        fn receive_packet(receiver: Box<dyn DataLinkReceiver>,producer_tx : Sender<NetworkUser>,scanner_manager : Box<dyn ScannerManager>){
            
            loop{
    
                match receiver.next(){
    
                    Ok(packet) => {
            
                        let ethernet_packet = EthernetPacket::new(packet).unwrap_or_else(|| panic!("No se ha podido decodificar el paquete"));
            
                        //EVALUTE CORRECT PACKET TYPE
                        //MUST SEND THE USER
                        let user = scanner_manager.parse_packet(ethernet_packet); 
                        
                        producer_tx.send(user);
    
            
                    },
                    Err(_) => println!("No ha sido posible recibir una trama"),
                
                };
    
            }
    
        }

    }



