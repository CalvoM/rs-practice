use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::prelude::FromRawFd;

#[derive(Debug, Deserialize, Serialize)]
struct OriginInfo {
    pub archive: Option<String>,
    pub codename: Option<String>,
    pub version: Option<String>,
    pub origin: Option<String>,
    pub label: Option<String>,
    pub site: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct VersionInfo {
    pub id: u32,
    pub version: String,
    pub architecture: String,
    pub pin: Option<i64>,
    pub origins: Option<Vec<OriginInfo>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Version {
    pub candidate: Option<VersionInfo>,
    pub install: Option<VersionInfo>,
    pub current: Option<VersionInfo>,
}
#[derive(Debug, Deserialize, Serialize)]
struct PackageInfo {
    pub id: u32,
    pub name: String,
    pub architecture: String,
    pub mode: String,
    pub downgrade: Option<bool>,
    pub upgrade: Option<bool>,
    pub reinstall: Option<bool>,
    pub automatic: Option<bool>,
    pub candidate: Option<String>,
    pub install: Option<String>,
    pub current: Option<String>,
    pub versions: Option<Version>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Params {
    pub versions: Option<Vec<String>>,
    pub command: Option<String>,
    #[serde(rename = "unknown-packages")]
    pub unknown_packages: Option<Vec<String>>,
    pub packages: Option<Vec<PackageInfo>>,
    #[serde(rename = "search-terms")]
    pub search_terms: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRPCRequest {
    pub jsonrpc: String,
    pub id: Option<i32>,
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
    pub id: i32,
    pub result: Result,
}

const HELLO_METHOD: &str = "org.debian.apt.hooks.hello";
const _PACKAGE_LIST_METHOD: &str = "org.debian.apt.hooks.install.package-list";
const _STATISTICS_METHOD: &str = "org.debian.apt.install.statistics";
const _INSTALL_POST: &str = "org.debian.apt.install.post";
const _INSTALL_FAIL: &str = "org.debian.apt.install.fail";
const _SEARCH_PRE: &str = "org.debian.apt.search.pre";
const _SEARCH_POST: &str = "org.debian.apt.search.post";
const _SEARCH_FAIL: &str = "org.debian.apt.search.fail";

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
        println!("{}", rsp);
        send_jsonrpc_response(&mut out, rsp);
        let later_req = read_jsonrpc_request(&mut out);
        println!("{:?}", later_req.method);
    } else {
        eprintln!("APT Hook not found");
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

fn send_jsonrpc_response(f: &mut File, response: String) {
    let msg = format!("{}\n\n", response);
    f.write_all(msg.as_bytes()).expect("Proper writing");
}
