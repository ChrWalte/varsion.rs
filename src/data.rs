use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_bytes_from_disk(path_to_file: &str) -> Vec<u8> {
    let path = Path::new(path_to_file);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("failed reading [{}]: [{}]", path_to_file, why),
    };

    let mut contents = Vec::new();
    match file.read_to_end(&mut contents) {
        Ok(_) => {}
        Err(why) => panic!("failed reading [{}]: [{}]", path_to_file, why),
    }

    contents
}

pub fn read_string_from_disk(path_to_file: &str) -> String {
    let as_bytes = read_bytes_from_disk(path_to_file);
    let as_string = match String::from_utf8(as_bytes) {
        Ok(s) => s,
        Err(why) => panic!("failed reading [{}]: [{}]", path_to_file, why),
    };

    as_string
}

pub fn write_bytes_to_disk(path_with_filename: &str, data: &[u8]) {
    let path = Path::new(path_with_filename);
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("failed creating [{}]: [{}]", path_with_filename, why),
    };
    match file.write_all(data) {
        Ok(_) => {}
        Err(why) => panic!("failed writing [{}]: [{}]", path_with_filename, why),
    }
}

pub fn write_str_to_disk(path_with_filename: &str, data: &str) {
    write_bytes_to_disk(path_with_filename, data.as_bytes());
}
