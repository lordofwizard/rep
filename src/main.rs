mod file_config;
mod input;
mod read;

use file_config::config::Config;
use input::input_via_env;
use read::read_func::read;
use rep::*;

fn main(){
    let (query,filename,true,true) = input_via_env::input();
    let config = Config::new(query,filename,true,true);
    let output : String = search_num_sensitive(config.query.clone(),&read(&config));
    println!("{}",&output);
}
