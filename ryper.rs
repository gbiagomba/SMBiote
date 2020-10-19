/* 
Author: Gilles Biagomba
Program: smb-ryper.rs
Description: This program was designed to scxan a network for SMB, NBT, LLMNR and MSRCP
*/

use clap::{Arg, App};
use std::env;
use std::fs;

fn main() 
{
   let matches = App::new("SMB Ryper")
        .version("0.1.0")
        .author("madhattr")
        .about("Scanning SMB, NBT, LLMNR, and MS-RPC")
        .arg(Arg::with_name("file")
                 .short("f")
                 .long("file")
                 .takes_value(true)
                 .help("target file"))
        .arg(Arg::with_name("mode")
                 .short("m")
                 .long("mode")
                 .takes_value(true)
                 .help("Modes: Recon, Intrusion, Exploit"))
         .arg(Arg::with_name("username")
                 .short("u")
                 .long("username")
                 .takes_value(true)
                 .help("Username you want to use"))
         .arg(Arg::with_name("User list")
                 .short("U")
                 .long("user-list")
                 .takes_value(true)
                 .help("File with a list of usernames"))
         .arg(Arg::with_name("password")
                 .short("p")
                 .long("password")
                 .takes_value(true)
                 .help("Password you want to use"))
         .arg(Arg::with_name("Pass List")
                 .short("P")
                 .long("pass-list")
                 .takes_value(true)
                 .help("File with a list of passwords"))
        .get_matches();

   // Setting targets var
    let targets = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    // setting user list
    let userList = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

   // setting password list
    let passList = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let num_str = matches.value_of("num");
    match num_str 
    {
        None => println!("No idea what your favorite number is."),
        Some(s) => 
        {
            match s.parse::<i32>() 
            {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }

   let fees = 25_000;
   let salary:f64 = 35_000.00;
   println!("fees is {} and salary is {}",fees,salary);
}
