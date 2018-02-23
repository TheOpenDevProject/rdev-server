mod server;
mod filehandle;
use server::Server;
use filehandle::FileHandle;
fn main() {
    let server_manager = Server::new();

   
    let test_file = FileHandle::open("test_files/sample.txt".to_string());
    println!("{:?}", test_file.read_out_buffer());

     server_manager.await_incoming_connection(test_file.read_out_buffer());

}
