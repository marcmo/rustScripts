use std::path::Path;

fn does_file_exist(file: &str) -> bool {
    use std::fs::metadata;
    return metadata(Path::new(file)).map(|m| m.is_file()).unwrap_or(false)
}

fn main() {
    use std::io::prelude::*;
    use std::fs;
    use std::fs::File;
    use std::vec::Vec;
    use std::error::Error;
    use std::io::BufReader;


    let mut cleaned : Vec<String> = Vec::new();
    let mru_file = ".vim_mru_files";
    let backup_file = ".vim_mru_files_backup";
    fs::copy(mru_file, backup_file).unwrap();

    let mut rm_cnt = 0;
    let mru_path = Path::new(mru_file);
    {
        let file = BufReader::new(File::open(&mru_path).unwrap());
        for line in file.lines().filter_map(|result| result.ok()) {

            let s = &line[..];
            if (!line.starts_with('#')) && !does_file_exist(s) {
                print!("{} ==> does not exists, CLEAN\n", line);
                rm_cnt = rm_cnt + 1;
            } else {
                cleaned.push(line.to_string());
                cleaned.push("\n".to_string());
            }
        }
    }
    let mut file = match File::create(&mru_path) {
        Err(why) => panic!("couldn't create {}: {}",
                           mru_path.display(),
                           Error::description(&why)),
        Ok(file) => file,
    };
    let joined : String = cleaned.iter().fold(String::new(), |r, c| r + c);
    match file.write_all(joined.as_bytes()) {
    // match file.write_all(cleaned.join("")) {
        Err(why) => {
            panic!("couldn't write to {}: {}",
                           mru_path.display(),
                                               Error::description(&why))
        },
        Ok(_) => println!("wrote cleaned file, {} entries removed", rm_cnt),
    }
}
