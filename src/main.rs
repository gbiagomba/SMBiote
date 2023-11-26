/* 
Author: Gilles Biagomba
Program: smb-ryper.rs
Description: This program was designed to scan a network for SMB, NBT, LLMNR and MSRCP
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use smbover::{SMBScanner, Auth};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <host> <username> <password>", args[0]);
        println!("       {} -f <file> <username> <password>", args[0]);
        std::process::exit(1);
    }

    let username = &args[args.len() - 2];
    let password = &args[args.len() - 1];

    if args[1] == "-f" {
        if args.len() != 5 {
            println!("Usage: {} -f <file> <username> <password>", args[0]);
            std::process::exit(1);
        }
        let file_path = &args[2];
        scan_from_file(file_path, username, password);
    } else {
        let host = &args[1];
        scan_single_host(host, username, password);
    }
}

fn scan_single_host(host: &str, username: &str, password: &str) {
    let scanner = SMBScanner::new();
    let auth = Auth::new(username.to_string(), password.to_string());
    
    match scanner.scan_host(host, &auth) {
        Ok(shares) => {
            println!("Shares on {}: {:?}", host, shares);
        }
        Err(err) => {
            eprintln!("Error scanning {}: {}", host, err);
        }
    }
}

fn scan_from_file(file_path: &str, username: &str, password: &str) {
    if let Ok(file) = File::open(file_path) {
        let scanner = SMBScanner::new();
        let auth = Auth::new(username.to_string(), password.to_string());

        for line in io::BufReader::new(file).lines() {
            if let Ok(host) = line {
                scan_single_host(&host, username, password);
            }
        }
    } else {
        eprintln!("Error opening file: {}", file_path);
    }
}
