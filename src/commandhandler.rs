use std::collections::HashMap;

type Command = fn() -> ();
pub struct CommandHandler {
    commands: HashMap<i32, Command>
}

impl CommandHandler {
     pub fn init() -> CommandHandler{
         let cmd_map = HashMap::new();
         
         cmd_map.insert(1, fn() -> (){
               println!("Open File Requested");
         });
           

         CommandHandler{
             commands: cmd_map
         }
     }

     pub fn handle_command(&mut self, command: i32){
         if self.commands.contains_key(command){
            let fnc = self.commands.get(command);
            fnc();
         }
     }
}