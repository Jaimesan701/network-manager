use crate::scanner_manager::{ArpScannerManager, IcmpScannerManager, ScannerManager};

pub enum ManagerType{
    ICMP,
    ARP
}


pub struct ManagerFactory;

impl ManagerFactory {
    fn new_shape(s: &ManagerType) -> Box<dyn ScannerManager> {

        match s {

            ManagerType::ICMP => Box::new(IcmpScannerManager {}),
            ManagerType::ARP => Box::new(ArpScannerManager {}),
            
        }

    }
}