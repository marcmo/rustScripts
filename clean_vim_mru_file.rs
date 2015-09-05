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
    fn replace_original() -> Result<i32, io::Error> {
        let mru_file = ".vim_mru_files";
        if !does_file_exist(mru_file) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                                      format!("{} seems not to exist", mru_file)))
        }
        let mru_path = Path::new(mru_file);
        let backup_file = ".vim_mru_files_backup";
        try!(fs::copy(mru_file, backup_file));
        let mut file = try!(File::create(&mru_path));
        let (joined, rm_cnt) = try!(get_cleaned_content(mru_path));
        try!(file.write_all(joined.as_bytes()));
        Ok(rm_cnt)
    }
    match replace_original() {
         Err(why) => {
             panic!("couldn't replace original: {}", Error::description(&why))
         },
         Ok(removed) => println!("wrote cleaned file, {} entries removed", removed),
    }
}
