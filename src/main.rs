mod file_config;
mod input;

use std::env;
use std::fs;
use file_config::config::Config;
use input::input_via_env;

fn main(){
    let (query,filename) = input_via_env::input();
    let config = Config::new(query,filename);
    config.print();

}