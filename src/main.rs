use std::env;
use std::fs;
use std::io::{self, Write};
use colored::*;

fn main() {
    //clears screen and puts cursor to top left
    print!("\x1B[2J\x1B[1;1H");  // ANSI escape sequence, might switch to crossterm

    println!("seaShell v0.1.0");
    
    // read config file to get info on colours of sea@ and the path

    // read shell config file to get info about path variables etc

    loop {
        let cwd = env::current_dir().unwrap();

        print!("{}", "sea@".truecolor(0, 230, 255));  // bright cyan
        print!("{} ", cwd.display().to_string().truecolor(0, 180, 255)); // gold/yellow
        
        //print!("{}", cwd.display().to_string().truecolor(112, 128, 144));

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "kill" {
            break;
        }

        //input checking with default command table
        // first define how the inputs will be + bnf for them

    }
}


fn read_config_file() {
    // fuck this for now ill do this shit later
}

fn readShellConfigFile(){
    // again fuck thsi shit we arent giving you custom commands until i finish the basic config
}