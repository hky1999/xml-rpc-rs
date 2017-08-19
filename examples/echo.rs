extern crate xml_rpc;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::{net, thread};
use xml_rpc::server::Server;
use xml_rpc::client::Client;
use xml_rpc::xmlfmt;

#[derive(Debug, Serialize, Deserialize)]
struct TestStruct {
    pub foo: i32,
    pub bar: String,
}

pub fn main() {
    let socket = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 8080);
    thread::spawn(move || Server::run(&socket).unwrap());
    let mut client = Client::new().unwrap();
    let req = TestStruct {
        foo: 42,
        bar: "baz".into(),
    };
    println!("Sending: {:?}", req);
    let uri = "http://localhost:8080/".parse().unwrap();
    let res: xmlfmt::Response<TestStruct> = client
        .call(
            uri,
            xmlfmt::Call {
                name: "foobar".into(),
                data: req,
            },
        )
        .unwrap();
    println!("Received: {:?}", res);
}