mod file_config;
use std::env;
use std::fs;
use file_config::config::Config;


fn main(){
    let config = Config::new(String::from("something"),String::from("sonething again"));
    config.print();
}