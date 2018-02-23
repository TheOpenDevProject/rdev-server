extern crate ws;
use self::ws::*;

mod server;
mod filehandle;
use server::{ServerConnection};
use filehandle::FileHandle;
use std::thread;
use std::thread::*;

fn main() {
    //let test_file = FileHandle::open("test_files/sample.txt".to_string());
    //println!("{:?}", test_file.read_out_buffer());

    thread::spawn(move ||{
         let x = ServerConnection::new_connection();
    }).join();
}
