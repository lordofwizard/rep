mod file_config;
mod input;
mod read;

use std::env;
use std::fs;
use file_config::config::Config;
use input::input_via_env;
use read::read_func::read;

fn main(){
    let (query,filename) = input_via_env::input();
    let config = Config::new(query,filename);
    //config.print();
    read(config);
}