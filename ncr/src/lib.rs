use clap::Parser;
use std::error::Error;
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{BufRead, BufReader, BufWriter, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    host: String,
    port: u16,
}

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "maetin")]
struct Opts {
    #[clap(required = true, help = "Host to connect to", index = 1)]
    host: String,
    #[clap(required = true, help = "Port to connect to", index = 2)]
    port: u16,
}

pub fn run(config: Config) -> MyResult<()> {
    let host_and_port = format!("{}:{}", config.host, config.port);
    let mut addrs = host_and_port.to_socket_addrs()?;
    if let Some(addr) = addrs.find(|x| x.is_ipv4()) {
        match TcpStream::connect(addr) {
            Ok(stream) => {
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                loop {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    print!("Send: {}", input);
                    writer.write_all(input.as_bytes())?;
                    writer.flush()?;
                    let mut buffer = String::new();
                    reader.read_line(&mut buffer)?;
                    print!("Received :{}", buffer);
                }
            }
            Err(e) => {
                println!("Faild to connect to {}: {}", addr, e);
                return Err(From::from(e))
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let opts = Opts::parse();

    Ok(Config { 
        host: opts.host,
        port: opts.port,
        }
    )
}   
