extern crate ws;

mod server;
mod filehandle;
mod commandhandler;
use commandhandler::CommandHandler;
use server::{ServerConnection};
use std::thread;

fn main() {
    //let test_file = FileHandle::open("test_files/sample.txt".to_string());
    //println!("{:?}", test_file.read_out_buffer());

    //Start the server on a separate thread from main()

    thread::spawn(move ||{
         ServerConnection::new_connection();
    }).join();
}
