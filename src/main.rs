use clap::{Arg, Command};
use std::fs::{create_dir_all, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{Command as Cmd};

/// List available shares using smbclient
fn list_shares(host: &str, username: &str, password: &str) -> Result<(), io::Error> {
    let output = Cmd::new("smbclient")
        .arg(format!("//{}/", host))
        .arg("-U")
        .arg(format!("{}%{}", username, password))
        .arg("-c")
        .arg("lmounts") // To list shares
        .output()?;

    if output.status.success() {
        let shares = String::from_utf8_lossy(&output.stdout);
        println!("Shares:\n{}", shares);
    } else {
        eprintln!(
            "Error listing shares: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

/// Download a file from an SMB share using smbclient
fn download_file(
    host: &str,
    share: &str,
    username: &str,
    password: &str,
    remote_file: &str,
    local_path: &Path,
) -> Result<(), io::Error> {
    let local_file = local_path.to_str().unwrap_or("downloaded_file");

    let output = Cmd::new("smbclient")
        .arg(format!("//{}/{}", host, share))
        .arg("-U")
        .arg(format!("{}%{}", username, password))
        .arg("-c")
        .arg(format!("get {} {}", remote_file, local_file))
        .output()?;

    if output.status.success() {
        println!("Downloaded file: {}", local_file);
    } else {
        eprintln!(
            "Error downloading file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

/// Recursively spider an SMB share and download files using smbclient
fn spider_directory(
    host: &str,
    share: &str,
    username: &str,
    password: &str,
    dir: &str,
    base_path: &Path,
) -> Result<(), io::Error> {
    let output = Cmd::new("smbclient")
        .arg(format!("//{}/{}", host, share))
        .arg("-U")
        .arg(format!("{}%{}", username, password))
        .arg("-c")
        .arg(format!("ls {}", dir))
        .output()?;

    if output.status.success() {
        let listing = String::from_utf8_lossy(&output.stdout);
        for line in listing.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 1 {
                let item_name = parts[0];
                let is_dir = line.contains("<DIR>");

                let remote_path = format!("{}/{}", dir, item_name);
                let local_path = base_path.join(item_name);

                if is_dir {
                    // Recursively download directories
                    create_dir_all(&local_path)?;
                    println!("Created directory: {:?}", local_path);
                    spider_directory(
                        host,
                        share,
                        username,
                        password,
                        &remote_path,
                        &local_path,
                    )?;
                } else {
                    // Download the file
                    download_file(host, share, username, password, &remote_path, &local_path)?;
                }
            }
        }
    } else {
        eprintln!(
            "Error listing directory: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

/// Spider SMB shares
fn spider_shares(host: &str, username: &str, password: &str) -> Result<(), io::Error> {
    let output = Cmd::new("smbclient")
        .arg(format!("//{}/", host))
        .arg("-U")
        .arg(format!("{}%{}", username, password))
        .arg("-c")
        .arg("lmounts") // List shares
        .output()?;

    if output.status.success() {
        let shares = String::from_utf8_lossy(&output.stdout);
        for share in shares.lines() {
            let share_name = share.trim();
            let base_path = Path::new("./downloads").join(share_name);

            // Create local folder for share
            create_dir_all(&base_path)?;
            println!("Spidering share: {}", share_name);

            // Start spidering the root directory of the share
            spider_directory(host, share_name, username, password, "", &base_path)?;
        }
    } else {
        eprintln!(
            "Error listing shares: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

/// Scan hosts from file
fn scan_hosts_from_file(
    file_path: &str,
    username: &str,
    password: &str,
    spider: bool,
) -> Result<(), io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(host) = line.as_deref() {
            if spider {
                if let Err(err) = spider_shares(host, username, password) {
                    eprintln!("Error spidering {}: {:?}", host, err);
                }
            } else {
                if let Err(err) = list_shares(host, username, password) {
                    eprintln!("Error listing shares on {}: {:?}", host, err);
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let matches = Command::new("SMBiote")
        .version("1.0")
        .author("Your Name")
        .about("SMBiote is an SMB scanning and exploitation tool")
        .arg(
            Arg::new("host")
                .short('H')
                .long("host")
                .value_parser(clap::value_parser!(String))
                .help("Target host"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_parser(clap::value_parser!(String))
                .help("File containing list of hosts"),
        )
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .value_parser(clap::value_parser!(String))
                .help("Username for SMB authentication"),
        )
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_parser(clap::value_parser!(String))
                .help("Password for SMB authentication"),
        )
        .arg(
            Arg::new("spider")
                .short('s')
                .long("spider")
                .help("Spider shares and download contents"),
        )
        .get_matches();

    let host = matches.get_one::<String>("host");
    let file = matches.get_one::<String>("file");
    let username = matches.get_one::<String>("username").map(String::as_str).unwrap_or("");
    let password = matches.get_one::<String>("password").map(String::as_str).unwrap_or("");
    let spider = matches.contains_id("spider");

    if let Some(host) = host {
        if spider {
            if let Err(err) = spider_shares(host, username, password) {
                eprintln!("Error spidering {}: {:?}", host, err);
            }
        } else {
            if let Err(err) = list_shares(host, username, password) {
                eprintln!("Error listing shares on {}: {:?}", host, err);
            }
        }
    } else if let Some(file_path) = file {
        if let Err(err) = scan_hosts_from_file(file_path, username, password, spider) {
            eprintln!("Error reading hosts file: {:?}", err);
        }
    } else {
        eprintln!("Please provide a host or a file with hosts");
    }
}