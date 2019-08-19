use futures::{future, prelude::*};
use libp2p::{
    identity,
    ping::{Ping, PingConfig},
    Multiaddr, PeerId, Swarm,
};
use std::env;

fn main() {
    let port = env::args().nth(1);
    let listen_addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/{}", port.clone().unwrap())
        .parse()
        .expect("invalid multiaddr");

    let keypair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());
    println!("Local peer id: {:?}", peer_id);

    let transport = libp2p::build_development_transport(keypair);

    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, peer_id);

    Swarm::listen_on(&mut swarm, listen_addr).unwrap();
    if let Some(addr_peer) = env::args().nth(2) {
        let remote_addr = addr_peer.clone();
        match addr_peer.parse::<Multiaddr>() {
            Ok(a) => {
                println!("Dial with : {:?}", remote_addr);
                match Swarm::dial_addr(&mut swarm, a) {
                    Ok(()) => println!("Dialed : {:?}", remote_addr),
                    Err(e) => println!("Dialing error : {:?}", e),
                }
            }
            Err(err) => println!("Failed to dial because : {:?}", err),
        }
    }

    let mut listening = false;
    tokio::run(future::poll_fn(move || -> Result<_, ()> {
        loop {
            match swarm.poll().expect("Error while polling") {
                Async::Ready(Some(e)) => println!("{:?}", e),
                Async::Ready(None) | Async::NotReady => {
                    if !listening {
                        println!("Listen on : {:?}", port);
                        listening = true;
                    }
                    return Ok(Async::NotReady);
                }
            }
        }
    }));
}
