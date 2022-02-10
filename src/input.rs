pub mod input_via_env{
    use std::env;
    use std::process;
    pub fn input() -> (String, String){
        let args : Vec<String> = env::args().collect();
        if args.len() == 1{
            eprintln!("Input expected");
            process::exit(1);
        }
        if args.len() == 2{
            eprintln!("file name not provided");
            process::exit(1);
        }
        (
            args.get(1)
                .expect("query not provided")
                .clone(),
            args.get(2)
                .expect("filename not given")
                .clone()
        )
    }
}