use std::{net::IpAddr, borrow::Cow, any::Any};

use pnet::{packet::{ethernet::{EthernetPacket, EtherTypes}, Packet, arp::{ArpPacket, ArpOperations}, ipv4::{Ipv4, Ipv4Packet}, ip::IpNextHeaderProtocols, icmp::{IcmpPacket, IcmpType, IcmpTypes}}, util::MacAddr};

use crate::{network_user::NetworkUser, packet_factory::PacketFactory, packet_initializer::PacketInitializer};



pub trait ScannerManager{

    fn create_packet(&self, my_ip : IpAddr, target_ip: IpAddr, my_mac: MacAddr) -> Vec<u8>;
    fn parse_packet(&self, packet : EthernetPacket) -> Option<NetworkUser>;

}



pub struct ArpScannerManager{

}
impl ArpScannerManager{

    pub fn new() -> ArpScannerManager{
        ArpScannerManager{

        }
    }

}

impl ScannerManager for ArpScannerManager{

    fn create_packet(&self, my_ip : IpAddr, target_ip: IpAddr, my_mac: MacAddr) -> Vec<u8> {

        let mut arp_packet = PacketFactory::create_arp_packet();
        PacketInitializer::initialize_arp_request_packet(&mut arp_packet, my_mac, my_ip, target_ip);

        let mut ethernet_packet = PacketFactory::create_ethernet_packet();
        let target_mac = MacAddr::new(0xff, 0xff, 0xff, 0xff,  0xff, 0xff);
        PacketInitializer::initialize_ethernet_packet(&mut ethernet_packet, my_mac,target_mac,EtherTypes::Arp, arp_packet.packet());

        let packet = ethernet_packet.packet().to_vec();

        packet

    }

    fn parse_packet(&self, packet : EthernetPacket) -> Option<NetworkUser> {

        if packet.get_ethertype() == EtherTypes::Arp{

            let arp_packet = ArpPacket::owned(packet.payload().to_vec()).unwrap();
            println!("{:?}",arp_packet);

            if (arp_packet.get_operation() == ArpOperations::Reply){   

                println!("{:?}",arp_packet);
                return Some(NetworkUser { ip: arp_packet.get_sender_proto_addr(), mac: arp_packet.get_sender_hw_addr() });
            
            }
        }
            
            None

    }
}

pub struct IcmpScannerManager{

}

impl IcmpScannerManager{

    pub fn new() -> IcmpScannerManager{

        IcmpScannerManager {  
            
        }

    }

}

impl ScannerManager for IcmpScannerManager{

    fn create_packet(&self, my_ip : IpAddr, target_ip: IpAddr, my_mac: MacAddr) -> Vec<u8>{
        
        let mut echo_request_packet = PacketFactory::create_echo_request_packet();
        PacketInitializer::initialize_echo_request_packet(&mut echo_request_packet);

        let mut ip_packet = PacketFactory::create_ip_packet();
        PacketInitializer::initialize_ip_packet(&mut ip_packet,my_ip,target_ip,echo_request_packet.packet());

        let mut ethernet_packet = PacketFactory::create_ethernet_packet();
        let target_mac = MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
        PacketInitializer::initialize_ethernet_packet(&mut ethernet_packet, my_mac,target_mac,EtherTypes::Ipv4, ip_packet.packet());

        let packet = ethernet_packet.packet().to_vec();

        packet
        
    }

    fn parse_packet(&self, packet : EthernetPacket) -> Option<NetworkUser> {
        
        if packet.get_ethertype() == EtherTypes::Ipv4{

            let mac_addr = packet.get_source();
            let ip_packet = Ipv4Packet::new(packet.payload()).unwrap();

            if ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Icmp{
                
                let ip_addr = ip_packet.get_source();
                let icmp_packet = IcmpPacket::new(ip_packet.payload()).unwrap();
                if icmp_packet.get_icmp_type() == IcmpTypes::EchoRequest{

                    return Some(NetworkUser { ip: ip_addr, mac: mac_addr});

                }
            }
        }

        None

    }
}