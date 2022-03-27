use futures::prelude::*;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{dial_opts::DialOpts, Swarm, SwarmEvent};
use libp2p::{identity, Multiaddr, PeerId};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;

    //create a ping network behaviour.
    //For illustrative purposes, the ping protocol is configured
    //to keep the connection alive, so a continuous sequence of pings
    //can be observed.
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    //Tell the swarm to listen to all interfaces and a random, OS-assigned port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    //Dial the peer identified by the multi-address given as the second command-line arguement, if any.
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }
    let mut count: i32 = 0;
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => {
                count += 1;
                println!("{:?} sent {} times", event, count)
            }
            _ => {}
        }
    }
}
