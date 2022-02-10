pub mod config{
    pub struct Config {
        pub query: String,
        pub filename: String,
    }
    impl Config {
        pub fn new(query_:String,filename_:String) -> Config{
            let query = query_.clone();
            let filename = filename_.clone();
            Config {
                query,
                filename
            }
        }
        pub fn print(&self){
            println!("{} {}" ,self.query,self.filename);
        }
    }
}