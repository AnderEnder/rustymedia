#[macro_use]
extern crate structopt;

extern crate env_logger;
extern crate futures_cpupool;
extern crate hyper;
extern crate pnet;
extern crate rustymedia;
extern crate tokio_core;

use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use structopt::StructOpt;
use rustymedia::*;
use rustymedia::error::*;
use rustymedia::dlna::discovery;
use rustymedia::dlna::server::ServerArgs;
use rustymedia::dlna::server::ServerFactory;
use rustymedia::root::Root;
use rustymedia::local::Object;

/// Serve and convert media
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "rustymedia")]
struct Args {
    /// Serving socket bind address. [default: [::]:4950]
    #[structopt(name = "address", long = "bind", short = "b", default_value = "[::]:4950")]
    flag_bind: std::net::SocketAddr,
    /// Map a local path to be served.
    /// The <mapping> argument should be in the form <name>=<path>
    /// where everything until the first `=` is treated as the name and the rest as the path.
    #[structopt(name = "local", long = "local", short = "l", raw(number_of_values = "1"))]
    flag_local: Vec<String>,
    /// Set the server name. [default: RustyMedia]
    #[structopt(name = "name", long = "name", short = "n", default_value = "RustyMedia")]
    flag_name: String,
    ///Server UUID. [default: 06289e13-a832-4d76-be0b-00151d449864]
     #[structopt(name = "uuid", long = "uuid", short = "u", default_value = "06289e13-a832-4d76-be0b-00151d449864")]
    flag_uuid: String,
}

fn find_public_addr(bind: SocketAddr) -> std::result::Result<SocketAddr, MediaError> {
    if bind.ip().is_unspecified() {
        for interface in pnet::datalink::interfaces() {
            if interface.is_loopback() {
                continue
            }

            for ipnetwork in interface.ips {
                return Ok(std::net::SocketAddr::new(ipnetwork.ip(), bind.port()));
            }
        }
        Err(MediaError::NotFound("Could not find public address! Please pass --bind=<ip>:<port>".to_owned()))
    } else {
        Ok(bind)
    }

}

fn result_main() -> Result<()> {
    let args = Args::from_args();

    let mut root = Root::new();

    for mapping in args.flag_local {
        //         let i = mapping.find('=').ok_or(Err(MediaError::NotFound("No `=` found in --local mapping".to_owned())))?;
        let i = mapping.find('=').expect("No `=` found in --local mapping");

        root.add(Object::new_root(
            mapping[..i].to_string(), mapping[i+1..].to_string())?);
    }

    if root.is_empty() {
        return Err(MediaError::NotFound("No folders configured.".to_owned()).into());
    }

    let root = Arc::new(root);

    let addr = find_public_addr(args.flag_bind)?;

    let handle: Arc<Mutex<Option<tokio_core::reactor::Remote>>> =
        Arc::new(std::sync::Mutex::new(None));

    let service_handle = handle.clone();
    let service = ServerFactory::new(
        ServerArgs {
            uri: format!("http://{}", addr),
            root: root.clone(),
            remote: move || service_handle.lock().unwrap().as_ref().unwrap().clone(),
            name: args.flag_name,
            uuid: args.flag_uuid,
        });

    let server = hyper::server::Http::new()
        .bind(&addr, service).unwrap();

    *handle.lock().unwrap() = Some(server.handle().remote().clone());

    println!("Listening on http://{}/", addr);
    discovery::schedule_presence_broadcasts(server.handle(), addr);
    server.run()?;
    println!("Done.");

    Ok(())
}

fn main() {
    env_logger::init().expect("Failed to init env_logger");
    if let Err(err) = result_main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
