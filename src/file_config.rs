pub mod config{

    pub struct Config {
        pub query: String,
        pub filename: String,
        case_sensitive: bool,
        num_sensitive : bool
    }
    
    impl Config {
        pub fn new(query_:String,filename_:String,b1:bool,b2:bool)-> Config{
            let query = query_.clone();
            let filename = filename_.clone();

            let case_sensitive = b1;
            let num_sensitive = b2;
            Config {
                query,
                filename,
                case_sensitive,
                num_sensitive
            }
        }
        #[allow(dead_code)]
        pub fn print(&self){
            println!("{} {}" ,self.query,self.filename);
        }
    }
}
