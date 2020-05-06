use std::net::TcpStream;
use rustbolt_protocol::auth_logon_challenge::{AuthLogonChallengeReq, AuthLogonChallengeRes};
use rustbolt_common::net::tcp_threaded_server::TcpThreadedServer;

fn main() {
    let _req: AuthLogonChallengeReq = AuthLogonChallengeReq::new();
    let _server = TcpThreadedServer::start(String::from("0.0.0.0"), 3724, |stream| -> std::io::Result<()> {
        println!("Received a stream");
        true
    });
}
