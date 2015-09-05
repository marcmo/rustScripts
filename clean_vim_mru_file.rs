use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;
use std::vec::Vec;
use std::error::Error;

fn does_file_exist(file: &str) -> bool {
    std::fs::metadata(Path::new(file)).map(|m| m.is_file()).unwrap_or(false)
}
fn get_cleaned_content(mru_path : &Path) -> Result<(String, i32), io::Error> {
    let mut cleaned : Vec<String> = Vec::new();
    let mut rm_cnt = 0;
    let content = BufReader::new(try!(File::open(&mru_path)));
    for line in content.lines().filter_map(|result| result.ok()) {

        let s = &line[..];
        if (!line.starts_with('#')) && !does_file_exist(s) {
            print!("{} ==> does not exists, CLEAN\n", line);
            rm_cnt = rm_cnt + 1;
        } else {
            cleaned.push(line.to_string());
        }
    }
    let firstline : String = cleaned[0].to_string();
    let joined : String = cleaned[1..].iter().fold(firstline, |r, c| r + "\n" + c);
    Ok((joined, rm_cnt))
}

fn main() {

    let mru_file = ".vim_mru_files";
    let mru_path = Path::new(mru_file);
    let backup_file = ".vim_mru_files_backup";
    let res = match fs::copy(mru_file, backup_file){
        Err(why) => panic!("couldn't create backup: {}", Error::description(&why)),
        Ok(r) => r,
    };
    let mut file = match File::create(&mru_path) {
        Err(why) => panic!("couldn't create {}: {}", mru_file,
                           Error::description(&why)),
        Ok(file) => file,
    };
    let (joined, rm_cnt) = get_cleaned_content(mru_path).unwrap();
    match file.write_all(joined.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", mru_file,
                                               Error::description(&why))
        },
        Ok(_) => println!("wrote cleaned file, {} entries removed", rm_cnt),
    }
}
