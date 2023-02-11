use crate::scanner_manager::{ArpScannerManager, IcmpScannerManager, ScannerManager};

pub enum ManagerType{
    ICMP,
    ARP
}


pub struct ManagerFactory;

impl ManagerFactory {

    pub fn new_manager(s: &ManagerType) -> Box<dyn ScannerManager + Send> {
        
        match s {

            ManagerType::ICMP => Box::new(IcmpScannerManager {}),
            ManagerType::ARP => Box::new(ArpScannerManager {}),
            
        }

    }
}