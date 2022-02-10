use std::env;
use std::process;
pub mod input_via_env{
    pub(crate) fn input() -> ((String, String)){
        let args : Vec<String> = std::env::args().collect();
        if args.len() == 1{
            println!("Input expected");
            std::process::exit(1);
        }
        if args.len() == 2{
            println!("file name not provided");
            std::process::exit(1);
        }
        (args.get(1).expect("query not provided").clone(),args.get(2).expect("filename not given").clone())
    }
}