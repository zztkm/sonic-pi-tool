use std::net::UdpSocket;
use std::net;
use rosc::{OscPacket, OscMessage, OscType};
use rosc::encoder;

/// Takes a string of Sonic Pi source code and sends it to the Sonic Pi server,
/// which is presumed to be listening on UDP port 4557.
///
/// We don't expect to recieve anything, so we bind to 0.0.0.0:0, which prompts
/// the OS to assign us an arbitrary unused port.
///
pub fn run_code(source: String) {
    let localhost = net::Ipv4Addr::new(127, 0, 0, 1);
    let s_pi_addr = net::SocketAddrV4::new(localhost, 4557);
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    let client_name = OscType::String("SONIC_PI_TOOL".to_string());
    let osc_source = OscType::String(source);

    let msg = &OscPacket::Message(OscMessage {
        addr: "/run-code".to_string(),
        args: Some(vec![client_name, osc_source]),
    });
    let msg_buf = encoder::encode(msg).unwrap();

    socket.send_to(&msg_buf, s_pi_addr).unwrap();
}
