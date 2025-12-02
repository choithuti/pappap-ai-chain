// src/network/p2p.rs
use libp2p::{
    gossipsub, identity, noise, tcp, yamux,
    swarm::{NetworkBehaviour, SwarmEvent},
    PeerId, Swarm,
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
    // [FIX] Đưa receiver vào trong struct để quản lý luồng
    command_rx: mpsc::UnboundedReceiver<Vec<u8>>, 
}

impl P2PNode {
    pub async fn new(
        local_key: identity::Keypair, 
        peer_count: Arc<AtomicUsize>
    ) -> Result<(Self, mpsc::UnboundedSender<Vec<u8>>, PeerId), Box<dyn Error>> {
        let local_peer_id = PeerId::from(local_key.public());
        
        // Setup Gossip & Identify (Giữ nguyên code cũ)
        let topic = gossipsub::IdentTopic::new("pappap-mainnet");
        let gossip_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .build()
            .map_err(|e| format!("Config error: {}", e))?;

        let behaviour = PappapBehaviour {
            gossipsub: gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(local_key.clone()),
                gossip_config,
            )?,
            identify: libp2p::identify::Behaviour::new(
                libp2p::identify::Config::new("pappap/0.8.0".into(), local_key.public())
            ),
        };

        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
            .with_behaviour(|_| behaviour)?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        swarm.listen_on("/ip4/0.0.0.0/tcp/9000".parse()?)?;
        
        // Subscribe topic
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // [FIX] Tạo channel tại đây và trả về Sender cho Main
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        Ok((Self { swarm, topic, peer_count, command_rx: cmd_rx }, cmd_tx, local_peer_id))
    }

    /// Vòng lặp chính xử lý cả Network Event và Command từ Chain
    pub async fn run(&mut self) {
        println!("🌐 P2P EVENT LOOP STARTED");
        loop {
            tokio::select! {
                // 1. Xử lý sự kiện mạng (Swarm)
                event = self.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => println!("👂 Listening on {:?}", address),
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            self.peer_count.fetch_add(1, Ordering::Relaxed);
                            println!("🤝 Connected: {:?}", peer_id);
                        },
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            self.peer_count.fetch_sub(1, Ordering::Relaxed);
                            println!("🔌 Disconnected: {:?}", peer_id);
                        },
                        SwarmEvent::Behaviour(PappapBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                            println!("📩 Gossip Message from {:?}", message.source);
                            // TODO: Forward message to Mempool/Chain validation
                        },
                        _ => {}
                    }
                }
                // 2. Xử lý lệnh từ Chain (Broadcast Block)
                Some(data) = self.command_rx.recv() => {
                    if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(self.topic.clone(), data) {
                        println!("❌ Broadcast Failed: {:?}", e);
                    } else {
                        println!("📡 Block Broadcasted to Network");
                    }
                }
            }
        }
    }
}
