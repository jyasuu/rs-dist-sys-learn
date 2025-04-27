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
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    id: u64, // Using u64 for IDs
    src: String,
    dest: String,
    body: Body, // Embed the Body struct
}


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;
    
    let m: Message = serde_json::from_str(&buffer)?;
    // println!("{:?}",m);
    // println!("{:?} , {:?}",buffer,m);
    let res = serde_json::to_string(&m)?;
    io::stdout().write_all(res.as_bytes())?;

    Ok(())
}


// fn hello() {
//     let str = r#"
//     {"echo": "Hello, world!"}
//     "#;
//     println!("{}",str);
// }

