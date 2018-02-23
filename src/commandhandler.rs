use filehandle::FileHandle;
#[derive(Debug)]
pub struct CommandHandler {}
impl CommandHandler {
    pub fn init() -> CommandHandler {
        CommandHandler {}
    }

    pub fn handle_command(&mut self, command: &str) -> String {
        let command_fnc = match command {
            "open" => Box::new(|| -> String {
                println!("Open File Command Handled");
                let mut active_file = FileHandle::open("test_files/sample.txt".to_string());
                active_file.read_out_buffer()
            }) as Box<Fn() -> String>,

            &_ => Box::new(|| -> String { "error".to_string() }) as Box<Fn() -> String>,
        };

        let x = command_fnc();
        x
    }
}
