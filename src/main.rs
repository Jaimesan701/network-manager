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
  

   let scanner = Scanner::new("enp2s0",ManagerType::ARP).unwrap_or_else(|| panic!("Error with the interface"));
   scanner.perform_scanner();
   

}
