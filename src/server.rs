extern crate ws;
use self::ws::*;




pub struct ServerConnection{
    pub out: Sender
}

impl ServerConnection{
       pub fn new_connection() -> Result<()>{
            let x = listen("127.0.0.1:1111", |out|{
                    ServerConnection{ out: out}
            });
            x
        }
}



 impl Handler for ServerConnection {
        fn on_message(&mut self, msg: Message) -> Result<()> {
            println!("Server got message '{}'. ", msg);
            self.out.send(Message::from("Test Return Message".to_string()))
        }

        fn on_close(&mut self, code: CloseCode, reason: &str) {
            println!("WebSocket closing for ({:?}) {}", code, reason);
            println!("Shutting down server after first connection closes.");
            self.out.shutdown().unwrap();
        }
 }