nwkcalc
===

Tool for calculating IPv4 network configuration values.

## Description

The following tools exist to calculate IP addresses for IPv4 networks.
- On Linux
  - ipcalc
  - ipcount
  - sipcalc
- Web site tools

Although our tool is not as good as these tools, I am developing it with the following goals
- Easy to use from the command line.
- Can be used on multiple platforms.

## VS.

A feature not present in other tools is the ability to output an enumeration of IP addresses.
This feature can be used, for example, to create a script to specify IP addresses in bulk.

Output
```bash
$ nwkcalc -e 192.168.1.1/24 > iplist.txt
$ cat iplist.txt
192.168.1.1/255.255.255.0
192.168.1.2/255.255.255.0
  :
192.168.1.253/255.255.255.0
192.168.1.254/255.255.255.0
```

## Usage

The command line arguments are currently under development and are subject to change.
```
nwkcalc [OPTIONS] <ADDRESS>

ARGS:
    <ADDRESS>    IP address string (IPv4 only)

OPTIONS:
    -e, --enumerate    Enumerated display of IP addresses
    -h, --help         Print help information
        --prefix       Prefix display of subnet masks when displaying IP address enumeration
    -V, --version      Print version information
```

Displays network information based on parameters (IP addresses).
```bash
$ nwkcalc 192.168.1.1/255.255.255.0
```
Output
```
Address:   192.168.1.1
NetMask:   255.255.255.0 (=24)
Network:   192.168.1.0/24
HostMin:   192.168.1.1
HostMax:   192.168.1.254
Broadcast: 192.168.1.255
Hosts/Net: 254
```

Subnet mask can also be prefixed notation (1-32).
```bash
$ nwkcalc 192.168.1.1/24
```
出力
```
Address:   192.168.1.1
NetMask:   255.255.255.0 (=24)
Network:   192.168.1.0/24
HostMin:   192.168.1.1
HostMax:   192.168.1.254
Broadcast: 192.168.1.255
Hosts/Net: 254
```

With the '-e' option, all IP addresses in the network based on the parameter (IP address) can be displayed in enumeration.
```bash
$ nwkcalc -e 192.168.1.1/24
```
Output
```
192.168.1.1/255.255.255.0
192.168.1.2/255.255.255.0
  :
192.168.1.253/255.255.255.0
192.168.1.254/255.255.255.0
```

With the '--prefix' option, the subnet mask can be prefixed (1-32) values in the enumeration display.
```bash
$ nwkcalc -e --prefix 192.168.1.1/24
```
Output
```
192.168.1.1/24
192.168.1.2/24
  :
192.168.1.253/24
192.168.1.254/24
```

## Install

### Environment in which the system is confirmed to work

Confirmed that the software works in the following environments.
- macOS Monterrey (12.4) 
- Windows 10 (Version 21H2)
- Linux (Debian, Ubuntu...)

### Steps

1. Install the Rust development environment. (For specific installation instructions, please refer to the following sites, etc.)  
[Install Rust \- Rust Programming Language](https://www.rust-lang.org/en-US/tools/install)

2. Clone this repository on an arbitrary directory.
```bash
$ git clone https://github.com/ma-kappa/nwkcalc.git
```

3. Go to the cloned local repository directory and build.
```bash
$ cd nwkcalc
$ cargo build --release
```

4. Install with the following command.
```bash
$ cargo install --path .
```
> The installation directory is directly under $HOME/.cargo/bin (%USERPROFILE%¥.cargo¥bin on Windows).

5. Verify that it runs successfully in n the command line shell environment.
```bash
$ nwkcalc 192.168.1.1/24
```

## License

See the [LICENSE file](./LICENSE) file in the current directory.


