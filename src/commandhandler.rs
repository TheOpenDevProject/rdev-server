use std::collections::HashMap;
#[derive(Debug)]
pub struct CommandHandler {
   
}
impl CommandHandler {

     pub fn init() -> CommandHandler{
         CommandHandler{}
     }

     pub fn handle_command(&mut self, command: &str){
         let command_fnc = match command {
             "open" => Box::new(||{
                 println!("Open File Command Handled");
             }) as Box<Fn()>,
             &_ => Box::new(||{
                 println!("Open File Command Handled");
             }) as Box<Fn()>
         };
         command_fnc();
     }
}