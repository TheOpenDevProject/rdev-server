extern crate ws;
use self::ws::listen;
pub struct Server{
    listen_ip: String,
    listen_port: String,
    ssl: bool,
    max_connections: i16
}

impl Server {
    pub fn new() -> Server{
         Server{
             listen_ip: "0.0.0.0".to_string(),
             listen_port: "3395".to_string(),
             ssl: false,
             max_connections: 32
         }
     }

   pub fn await_incoming_connection(self,f_handle: String) -> (){
         listen("127.0.0.1:3012", |out| {
             move |msg|{
             out.send()
             }
      
             
        });
     }
}