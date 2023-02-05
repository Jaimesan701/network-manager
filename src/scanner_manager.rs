use pnet::{packet::{ethernet::{EthernetPacket, EtherTypes}, Packet, arp::ArpPacket, icmp::IcmpPacket}, util::MacAddr};

use crate::{network_user::NetworkUser, packet_factory::PacketFactory, packet_initializer::PacketInitializer};



pub trait ScannerManager{

    fn create_packet(&self) -> &[u8];
    fn parse_packet(&self, packet : EthernetPacket) -> NetworkUser;

}



pub struct ArpScannerManager{

}

impl ScannerManager for ArpScannerManager{

    fn create_packet(&self) -> &[u8] {

        let mut arp_packet = PacketFactory::create_arp_packet();
        PacketInitializer::initialize_arp_request_packet(&mut arp_packet, my_mac, my_ip, target_ip);

        let mut ethernet_packet = PacketFactory::create_ethernet_packet();
        let target_mac = MacAddr::new(0xff, 0xff, 0xff, 0xff,  0xff, 0xff);
        PacketInitializer::initialize_ethernet_packet(&mut ethernet_packet, my_mac,target_mac,EtherTypes::Arp, arp_packet.packet());
        ethernet_packet.packet()

    }

    fn parse_packet(&self, packet : EthernetPacket) -> NetworkUser {
        
        let arp_packet = ArpPacket::owned(packet.packet()).unwrap();
        NetworkUser { ip: arp_packet.get_sender_proto_addr(), mac: arp_packet.get_sender_hw_addr() }
    }
}

pub struct IcmpScannerManager{

}

impl ScannerManager for IcmpScannerManager{

    fn create_packet(&self) -> &[u8] {
        
        let mut echo_request_packet = PacketFactory::create_echo_request_packet();
        PacketInitializer::initialize_echo_request_packet(&mut echo_request_packet);

        let mut ip_packet = PacketFactory::create_ip_packet();
        PacketInitializer::initialize_ip_packet(&mut ip_packet,my_ip,target_ip,echo_request_packet.packet());

        let mut ethernet_packet = PacketFactory::create_ethernet_packet();
        let target_mac = MacAddr::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
        PacketInitializer::initialize_ethernet_packet(&mut ethernet_packet, my_mac,target_mac,EtherTypes::Ipv4, ip_packet.packet());

        ethernet_packet.packet()
        
    }

    fn parse_packet(&self, packet : EthernetPacket) -> NetworkUser {
        
        if packet.get_ethertype() == EtherTypes::Ipv4{

            todo!()
        }

    }
}