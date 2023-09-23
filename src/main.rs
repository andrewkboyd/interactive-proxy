use std::env;
use std::net::{TcpListener, TcpStream};

struct Args<'a> {
    listener: &'a str,
}

fn handle_client(_stream: TcpStream) {}

fn main() -> std::io::Result<()> {
    let mut args: Vec<_> = env::args_os().collect();
    args.remove(0);
    let mut parsedArgs = Args {
        listener: "127.0.0.1:8080",
    };

    while let Some(arg) = args.pop_front()? {
        println!("{arg}");
        match arg {
            "-l" => parsedArgs.listener = args.pop_front(),
            _ => println!("unknown arg given"),
        }
    }

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        println!("Connection from peer @ {}:{}", addr.ip(), addr.port());
        handle_client(stream);
    }
    Ok(())
}
