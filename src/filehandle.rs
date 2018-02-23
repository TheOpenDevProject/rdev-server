use std::env;
use std::fs::File;
use std::io::prelude::*;
struct InMemoryFile {
    buffer: String,
    file_name: String,
    file_size: usize,
}

pub struct FileHandle {
    active_file: InMemoryFile,
}

impl FileHandle {
    pub fn open(p_file_name: String) -> FileHandle {
        let mut ifh = File::open(p_file_name).expect("FS: Cant find file");
        let mut lbuffer = String::new(); //Create a string to buffer the content into memory
        ifh.read_to_string(&mut lbuffer).expect(
            "Unable to read contents of file",
        );
        FileHandle {
            active_file: InMemoryFile {
                buffer: lbuffer,
                file_name: "p_file_name".to_string(),
                file_size: 0,
            },
        }
    }

    pub fn read_out_buffer(self) -> String {
        self.active_file.buffer
    }
}
