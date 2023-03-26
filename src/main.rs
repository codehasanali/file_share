use std::net::IpAddr;

mod connectioninfo;
mod server_file;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}


fn main() {
  

  use local_ip_address::local_ip;

  let args:Args=Args::parse();

  let ip:IpAddr=local_ip().unwrap();
  let port:u16=8080;

  let file:String =String::from(args.filename);

  connectioninfo::display(ip, port);

  server_file::serve_file(ip, port, file)
}
