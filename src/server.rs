use std::net::TcpListener;
use std::io::Read;


pub struct Server{
  addr: String,
}

impl Server{
  pub fn new (addr: String) ->Self {
    Self{ addr }
  }

  pub fn run(self) {
    println!("Listening on address {}", self.addr);

    let listener = TcpListener::bind(&self.addr).unwrap();

    dbg!(&listener); 

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          println!("Listener is Ok");

          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => println!("{}", String::from_utf8_lossy(&buffer)),
            Err(e) => println!("Error reading {}", e),
          }
        },
        Err(e) => {
          println!("Listener ERROR {}", e);
        }
      }
    }
    
  }
}