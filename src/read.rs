use std::fs;
pub mod read_func{
    use std::fs;
    use std::path::Path;
    use crate::Config;


    pub fn read(conf : Config){
        let file_path  = Path::new(&conf.filename);
        let contains : String = fs::read_to_string(file_path)
                                    .expect("Something went wrong while reading the file");


        println!("{}",contains);
    }
}