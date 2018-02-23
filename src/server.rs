extern crate ws;
use self::ws::*;
use CommandHandler;



pub struct ServerConnection{
    pub out: Sender,
    cmd_handler: CommandHandler
}

impl ServerConnection{
       pub fn new_connection() -> Result<()>{
            let x = listen("127.0.0.1:1111", |out|{
                    ServerConnection{ out: out, cmd_handler: CommandHandler::init()}
            });
            x
        }
        
        fn handle_incoming_command(&mut self, msg: &str) -> String{
            self.cmd_handler.handle_command(msg)
        }
}



 impl Handler for ServerConnection {
        fn on_message(&mut self, msg: Message) -> Result<()> {
            let output = self.handle_incoming_command(msg.as_text().unwrap());
            
            self.out.send(Message::from(output));
            Ok(())
        }

        fn on_close(&mut self, code: CloseCode, reason: &str) {
            println!("WebSocket closing for ({:?}) {}", code, reason);
            println!("Shutting down server after first connection closes.");
            self.out.shutdown().unwrap();
        }
 }