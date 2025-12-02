// src/network/p2p.rs
use libp2p::{
    gossipsub, identity, noise, tcp, yamux,
    swarm::{NetworkBehaviour, SwarmEvent},
    PeerId, Swarm, Multiaddr,
};
use libp2p::futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;
use std::error::Error;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

#[derive(NetworkBehaviour)]
struct PappapBehaviour {
    gossipsub: gossipsub::Behaviour,
    identify: libp2p::identify::Behaviour,
}

pub struct P2PNode {
    swarm: Swarm<PappapBehaviour>,
    topic: gossipsub::IdentTopic,
    pub peer_count: Arc<AtomicUsize>,
}

impl P2PNode {
    pub async fn new(
        local_key: identity::Keypair, 
        peer_count: Arc<AtomicUsize>
    ) -> Result<(Self, mpsc::UnboundedReceiver<Vec<u8>>, PeerId), Box<dyn Error>> {
        let local_peer_id = PeerId::from(local_key.public());
        
        // 1. Setup Gossipsub (Pub/Sub)
        let topic = gossipsub::IdentTopic::new("pappap-mainnet");
        
        let gossip_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .build()
            .map_err(|e| format!("Gossip config error: {}", e))?;

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossip_config,
        )?;

        // 2. Setup Identify (Để peer nhận ra nhau)
        let identify = libp2p::identify::Behaviour::new(
            libp2p::identify::Config::new("pappap/0.8.0".into(), local_key.public())
        );

        let mut behaviour = PappapBehaviour { gossipsub, identify };
        behaviour.gossipsub.subscribe(&topic)?;

        // 3. Build Swarm
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
            .with_behaviour(|_| behaviour)?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        // Listen on any available port (0)
        swarm.listen_on("/ip4/0.0.0.0/tcp/9000".parse()?)?;

        let (_, rx) = mpsc::unbounded_channel();
        Ok((Self { swarm, topic, peer_count }, rx, local_peer_id))
    }

    pub fn broadcast_block(&mut self, data: Vec<u8>) {
        if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(self.topic.clone(), data) {
            println!("❌ P2P Publish Error: {:?}", e);
        } else {
            println!("📡 Broadcasted Block to P2P Network");
        }
    }

    /// Hàm này chạy một bước (step) của vòng lặp sự kiện.
    /// Nó được gọi liên tục trong vòng lặp tokio::select! ở main.rs
    pub async fn run(&mut self) {
        match self.swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("👂 P2P Listening on {:?}", address);
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                self.peer_count.fetch_add(1, Ordering::Relaxed);
                println!("🤝 Peer Connected: {:?}", peer_id);
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                self.peer_count.fetch_sub(1, Ordering::Relaxed);
                println!("🔌 Peer Disconnected: {:?}", peer_id);
            }
            SwarmEvent::Behaviour(PappapBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                println!("📩 Received Gossip Message from {:?}", message.source);
                // Ở đây bạn có thể gửi message này về Chain để validate (thông qua channel khác nếu cần)
            }
            _ => {}
        }
    }
}
