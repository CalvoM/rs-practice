use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::prelude::FromRawFd;

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    pub versions: Option<Vec<String>>,
    pub command: Option<String>,
    #[serde(rename = "unknown-packages")]
    pub unknown_packages: Option<Vec<String>>,
    pub packages: Option<Vec<String>>,
    #[serde(rename = "search-terms")]
    pub search_terms: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRPCRequest {
    pub jsonrpc: String,
    pub id: Option<i64>,
    pub method: String,
    pub params: Params,
}
#[derive(Debug, Deserialize, Serialize)]
struct Result {
    pub version: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct JsonRPCHelloResponse {
    pub jsonrpc: String,
    pub id: i64,
    pub result: Result,
}

const HELLO_METHOD: &str = "org.debian.apt.hooks.hello";

fn main() {
    if let Ok(apt_socket) = env::var("APT_HOOK_SOCKET") {
        let fd = apt_socket.parse::<i32>().unwrap();
        let mut out = unsafe { File::from_raw_fd(fd) };
        let hello_req = read_jsonrpc_request(&mut out);
        if hello_req.method != HELLO_METHOD {
            eprintln!("my-apt-hook: invalid hello message");
            return;
        }
        let hello_rsp = JsonRPCHelloResponse {
            jsonrpc: "0.2".to_owned(),
            id: hello_req.id.unwrap(),
            result: Result {
                version: "0.2".to_owned(),
            },
        };
        let rsp: String = serde_json::to_string(&hello_rsp).unwrap();
        let msg = format!("{}\n\n", rsp);
        out.write(msg.as_bytes()).expect("Proper writing");
        let later_req = read_jsonrpc_request(&mut out);
        println!("{:?}", later_req);
    } else {
        eprintln!("APT Hook not found");
        return;
    }
}

fn read_jsonrpc_request(f: &mut File) -> JsonRPCRequest {
    let mut reader = BufReader::new(f);
    let mut msg_line: String = String::new();
    let mut empty_line: String = String::new();
    reader.read_line(&mut msg_line).unwrap();
    reader.read_line(&mut empty_line).unwrap();
    let msg: JsonRPCRequest = serde_json::from_str(msg_line.as_str()).unwrap();
    msg
}
