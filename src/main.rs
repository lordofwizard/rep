mod file_config;
mod input;
mod read;

use file_config::config::Config;
use input::input_via_env;
use read::read_func::read;
use rep::*;

fn main(){
    let (query,filename) = input_via_env::input();
    let config = Config::new(query,filename);
    let output : String = search(config.query.clone(),&read(&config));
    println!("{}",&output);
}