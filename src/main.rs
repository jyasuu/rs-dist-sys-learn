use std::io::{self, BufRead, Write};

use serde::{Deserialize, Serialize};
// use serde_json::Result;

// 2. Define structs matching the JSON structure
// Note: 'type' is a Rust keyword, so we rename it using serde attribute
#[derive(Serialize, Deserialize, Debug)]
struct Body {
    #[serde(rename = "type")] // Map JSON "type" field to Rust "message_type"
    message_type: String,
    node_id: String,
    node_ids: Vec<String>,
    msg_id: u64, // Using u64 for IDs, adjust if needed (e.g., usize, i64)
    in_reply_to: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: u64, // Using u64 for IDs
    src: String,
    dest: String,
    body: Body, // Embed the Body struct
}

#[derive(Serialize, Deserialize, Debug)]
struct ResMessage {
    src: String,
    dest: String,
    body: Body, // Embed the Body struct
}

fn main() -> io::Result<()> {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let mut m: Message = serde_json::from_str(&input)?;
                // let res_m = Message {
                //     id: 0,
                //     src: m.dest.clone(),
                //     dest: m.src,
                //     body: Body {
                //         message_type: m.body.message_type, // String::from("echo_ok"),
                //         node_id: m.dest.clone(),
                //         node_ids: m.body.node_ids,
                //         msg_id: m.body.msg_id,
                //         in_reply_to: Some(m.body.msg_id),
                //     },
                // };
                //println!("{:?}",m);
                // let res_m2 = ResMessage {
                //     src: m.dest.clone(),
                //     dest: m.body.node_id.clone(),
                //     body: Body {
                //         message_type: String::from("echo_ok"),
                //         node_id: m.body.node_id.clone(),
                //         node_ids: m.body.node_ids.clone(),
                //         msg_id: m.body.msg_id +1,
                //         in_reply_to: None,
                //     },
                // };
                // m.src = m.dest.clone();
                let res = serde_json::to_string(&m)?;
                //println!("{:?}",m);
                //println!("{:#?}",res);
                io::stdout().write_all(res.as_bytes())?;
            }
            Err(error) => println!("error: {}", error),
        }
        io::stdout().flush().unwrap();
    }
}
