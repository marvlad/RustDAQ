/// Basic ServiceDiscovery with zmq 

use tokio::{
    net::UdpSocket,
    time::{sleep, Duration},
};
use serde::{Serialize, Deserialize};
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use zmq;
use anyhow::Result;


/// JSON structure of service discovery messages
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceMessage {
    pub msg_type: String,
    pub msg_value: String,
    pub msg_id: u64,
    pub msg_time: String,
    pub remote_port: u16,
    pub status: String,
    pub uuid: String,
}


/// Query the C++ or Rust status server via ZeroMQ
pub fn query_status() -> Option<String> {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REQ).ok()?;

    // Connect to localhost:6666 (your status server)
    socket.connect("tcp://127.0.0.1:6666").ok()?;

    // Send JSON command
    let msg = r#"{"msg_type":"Command","msg_value":"Status"}"#;
    socket.send(msg, 0).ok()?;

    // Poll for up to 1 second
    let mut items = [socket.as_poll_item(zmq::POLLIN)];
    zmq::poll(&mut items, 1000).ok()?;

    if items[0].is_readable() {
        let reply = socket.recv_string(0).ok()?;
        return reply.ok();
    }

    None
}


/// MULTICAST SENDER ---------------------------------------------------
pub async fn run_multicast_sender() -> Result<()> {
    let multicast_addr = "239.192.1.1:5000";
    let local_bind = "0.0.0.0:0";

    let socket = UdpSocket::bind(local_bind).await?;
    socket.join_multicast_v4("239.192.1.1".parse()?, "0.0.0.0".parse()?)?;

    let uuid = Uuid::new_v4();
    let mut msg_id: u64 = 0;

    println!(" Multicast Sender started — sending to {}", multicast_addr);

    loop {
        msg_id += 1;

        let status = query_status().unwrap_or("N/A".to_string());

        let payload = json!({
            "msg_type": "Service Discovery",
            "msg_value": "RustService",
            "msg_id": msg_id,
            "msg_time": Utc::now().to_rfc3339(),
            "remote_port": 6666,
            "status": status,
            "uuid": uuid.to_string()
        });

        let bytes = payload.to_string().into_bytes();

        socket.send_to(&bytes, multicast_addr).await?;

        println!(" Sent: {}", payload);

        sleep(Duration::from_secs(2)).await;
    }
}


/// MULTICAST RECEIVER ---------------------------------------------------
pub async fn run_multicast_receiver() -> Result<()> {
    let multicast_addr = "239.192.1.1:5000";

    let socket = UdpSocket::bind(multicast_addr).await?;
    socket.join_multicast_v4("239.192.1.1".parse()?, "0.0.0.0".parse()?)?;

    println!("Rust multicast receiver listening on {}", multicast_addr);

    let mut buf = vec![0u8; 2048];

    loop {
        let (len, sender) = socket.recv_from(&mut buf).await?;
        let raw = String::from_utf8_lossy(&buf[..len]).to_string();

        match serde_json::from_str::<ServiceMessage>(&raw) {
            Ok(msg) => {
                println!("From {} → {:#?}", sender, msg);
            }
            Err(_) => {
                println!(" Received non-JSON from {}: {}", sender, raw);
            }
        }
    }
}

