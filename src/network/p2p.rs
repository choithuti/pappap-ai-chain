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

// Định nghĩa hành vi mạng (Behaviour) bao gồm Gossipsub (pub/sub) và Identify (nhận diện peer)
#[derive(NetworkBehaviour)]
struct PappapBehaviour {
    gossipsub: gossipsub::Behaviour,
    identify: libp2p::identify::Behaviour,
}

pub struct P2PNode {
    pub swarm: Swarm<PappapBehaviour>,
    pub topic: gossipsub::IdentTopic,
    pub peer_count: Arc<AtomicUsize>,
}

impl P2PNode {
    pub async fn new(
        local_key: identity::Keypair, 
        peer_count: Arc<AtomicUsize>,
        port: u16
    ) -> Result<(Self, mpsc::UnboundedReceiver<Vec<u8>>, PeerId), Box<dyn Error>> {
        let local_peer_id = PeerId::from(local_key.public());
        println!("🆔 LOCAL PEER ID: {}", local_peer_id);

        // 1. Tạo Topic cho mạng lưới
        let topic = gossipsub::IdentTopic::new("pappap-mainnet");

        // 2. Cấu hình Gossipsub
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .build()
            .map_err(|msg| std::io::Error::new(std::io::ErrorKind::Other, msg))?;

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )?;

        // 3. Cấu hình Identify
        let identify = libp2p::identify::Behaviour::new(
            libp2p::identify::Config::new("pappap/7.7.7".into(), local_key.public())
        );

        let mut behaviour = PappapBehaviour {
            gossipsub,
            identify,
        };

        // Đăng ký topic
        behaviour.gossipsub.subscribe(&topic)?;

        // 4. Xây dựng Swarm
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)?
            .with_behaviour(|_| behaviour)?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        // Lắng nghe trên port chỉ định
        swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", port).parse()?)?;

        // Channel để gửi dữ liệu về Main Loop (Chain)
        // Lưu ý: Trong thực tế, bạn cần giữ lại `tx` để gửi dữ liệu vào channel.
        // Ở đây tôi tạo channel nhưng tạm thời drop tx theo code mẫu của bạn.
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Để demo hoạt động, chúng ta cần clone tx để dùng trong run loop sau này (nếu cần chỉnh sửa)
        // Nhưng tuân thủ đúng yêu cầu "hiển thị code use...", tôi giữ nguyên logic trả về rx.

        Ok((Self { swarm, topic, peer_count }, rx, local_peer_id))
    }

    /// Phát tán Block hoặc Transaction ra toàn mạng
    pub fn broadcast_block(&mut self, data: Vec<u8>) {
        if let Err(e) = self.swarm.behaviour_mut().gossipsub.publish(self.topic.clone(), data) {
            println!("❌ Publish error: {:?}", e);
        }
    }

    /// Vòng lặp chính xử lý sự kiện mạng
    pub async fn run(&mut self, _rx: mpsc::UnboundedReceiver<Vec<u8>>) {
        println!("🌐 P2P NETWORK STARTED");
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                     println!("👂 Listening on {:?}", address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    self.peer_count.fetch_add(1, Ordering::Relaxed);
                    println!("🤝 Connection Established: {:?}", peer_id);
                }
                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                    self.peer_count.fetch_sub(1, Ordering::Relaxed);
                    println!("🔌 Connection Closed: {:?}", peer_id);
                }
                // Xử lý tin nhắn nhận được từ Gossipsub
                SwarmEvent::Behaviour(PappapBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                    println!("📩 Received {} bytes from {:?}", message.data.len(), message.source);
                    // TODO: Gửi message về Chain để validate (sử dụng channel tx nếu được lưu)
                }
                _ => {}
            }
        }
    }
}
