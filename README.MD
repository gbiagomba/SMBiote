![alt tag](img/Firefly Design a dark, gritty logo for 'SMBiote' with a villainous, comic book feel. Draw inspiratio (1).jpg)

# SMBiote
SMBiote is a Rust-based SMB scanning tool that allows you to access and interact with SMB shares on remote hosts. It supports null session logins, lists available shares, spiders shares, and downloads their contents. You can supply a single host or a list of hosts from a file.

## Features
- List SMB shares on remote hosts
- Spider and download contents from shares
- Perform null session authentication
- Check password policies (to be implemented)

## Usage

### Command-line options

```bash
SMBiote 1.0
Your Name
SMBiote is an SMB scanning and exploitation tool

USAGE:
    smbiote [FLAGS] [OPTIONS]

FLAGS:
    -n, --null-session    Perform a null session

OPTIONS:
    -H, --host <host>          Target host
    -f, --file <file>          File containing list of hosts
    -u, --username <username>  Username for SMB authentication
    -p, --password <password>  Password for SMB authentication

### Examples

- List SMB shares on a single host:

```bash
smbiote --host 192.168.1.10 --username guest --password guest
```

- Perform a null session on a host:

```bash
smbiote --host 192.168.1.10 --null-session
```

- Scan a list of hosts from a file:

```bash
smbiote --file hosts.txt --username guest --password guest
```

## Installation

```bash
git clone https://github.com/yourusername/smbiote.git
cd smbiote
cargo build --release
```

## License

SMBiote is licensed under the [GPL-3.0 License](https://www.gnu.org/licenses/gpl-3.0.en.html).