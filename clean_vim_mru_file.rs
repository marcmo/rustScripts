use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;
use std::vec::Vec;
use std::error::Error;

static BACKUP_FILE: &'static str = ".vim_mru_files_backup";
static MRU_FILE: &'static str = ".vim_mru_files";

fn does_file_exist(path: &Path) -> bool {
    std::fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
}
fn get_cleaned_content(mru_path : &Path) -> Result<(String, i32), io::Error> {
    let mut cleaned : Vec<String> = Vec::new();
    let mut rm_cnt = 0;
    let reader = BufReader::new(try!(File::open(mru_path)));
    for line in reader.lines().filter_map(|result| result.ok()) {
        let s = &line[..];
        if (!line.starts_with('#')) && !does_file_exist(Path::new(s)) {
            print!("{} ==> does not exists, CLEAN\n", line);
            rm_cnt = rm_cnt + 1;
        } else {
            cleaned.push(line.to_string());
        }
    }
    if cleaned.len() > 0 {
        let firstline : String = cleaned[0].to_string();
        let joined : String = cleaned[1..].iter().fold(firstline, |r, c| r + "\n" + c);
        return Ok((joined, rm_cnt));
    }
    Err(io::Error::new(io::ErrorKind::Other, "empty cleaned stuff"))
}

fn main() {
    fn replace_original() -> Result<i32, io::Error> {
        let home = std::env::home_dir().expect("homedir not available");
        let mru_path_buf = Path::new(&home).join(MRU_FILE);
        let mru_path = mru_path_buf.as_path();
        if !does_file_exist(mru_path) {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                                      format!("{} seems not to exist", mru_path.display())))
        }
        let backup_path_buf = Path::new(&home).join(BACKUP_FILE);
        try!(fs::copy(mru_path, backup_path_buf));
        let (joined, rm_cnt) = try!(get_cleaned_content(&mru_path));
        let mut file = try!(File::create(mru_path));
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
