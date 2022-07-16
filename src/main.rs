use clap::Parser;
use regex::Regex;
use std::net::Ipv4Addr;
use ipnetwork::Ipv4Network;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Args {
    /// Enumerated display of IP addresses.
    #[clap(short, long, takes_value = false)]
    enumerate: bool,
    /// Prefix display of subnet masks when displaying IP address enumeration.
    #[clap(long, takes_value = false)]
    prefix: bool,
    /// IP address string (IPv4 only).
    address: String,
}

fn get_ipaddress_pattern1_regex() -> Regex {
    // Pattern 1 (xxx.xxx.xxx.xxx/xx)
    let re_addr = Regex::new(r"^((?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?))/([1-9]|(1|2)[0-9]|3[0-2]?)$").unwrap();
    return re_addr
}

fn parse_ipaddress_pattern1(args: &Args) -> Result<ipnetwork::Ipv4Network, &str> {
    // Pattern 1 (xxx.xxx.xxx.xxx/xx)
    let re_addr = get_ipaddress_pattern1_regex();
    let caps = re_addr.captures(&args.address).unwrap();
    let ip_prefix = match (caps[1].parse::<Ipv4Addr>(), caps[2].parse::<u8>()) {
        (Ok(addr), Ok(mask)) => {
            Ipv4Network::new(addr, mask).unwrap()
        },
        _ => {
            panic!("parse error")
        }
    };
    Ok(ip_prefix)
}

fn get_ipaddress_pattern2_regex() -> Regex {
    // Pattern 2 (xxx.xxx.xxx.xxx/xxx.xxx.xxx.xxx)
    let re_addr = Regex::new(r"^((?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?))/((?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?))$").unwrap();
    return re_addr
}

fn parse_ipaddress_pattern2(args: &Args) -> Result<ipnetwork::Ipv4Network, &str> {
    // Pattern 2 (xxx.xxx.xxx.xxx/xxx.xxx.xxx.xxx)
    let re_addr = get_ipaddress_pattern2_regex();
    let caps = re_addr.captures(&args.address).unwrap();
    let ip_prefix = match (caps[1].parse::<Ipv4Addr>(), caps[2].parse::<Ipv4Addr>()) {
        (Ok(addr), Ok(mask)) => {
            Ipv4Network::with_netmask(addr, mask).unwrap()
        },
        _ => {
            panic!("parse error")
        }
    };
    Ok(ip_prefix)
}

fn get_host_min_idx(ipv4: &Ipv4Network) -> u32 {
    match ipv4.prefix() {
        32 => 0,
        31 => 1,
        _ => 1 
    }
}

fn get_host_max_idx(ipv4: &Ipv4Network) -> u32 {
    match ipv4.prefix() {
        32 => 0,
        31 => ipv4.size() - 1,
        _ => ipv4.size() - 2
    }
}

fn print_ipv4_info(ipv4: &Ipv4Network) {
    let start_idx = get_host_min_idx(&ipv4);
    let end_idx = get_host_max_idx(&ipv4);

    println!("Address:   {}", ipv4.ip());
    println!("NetMask:   {}", format!("{} (={})", ipv4.mask(), ipv4.prefix()));
    println!("Network:   {}", if ipv4.prefix() < 32 { format!("{}/{}", ipv4.network(), ipv4.prefix()) } else { "-".to_string() });
    println!("HostMin:   {}", if ipv4.prefix() < 31 { format!("{}", ipv4.nth(start_idx).expect("panic!")) } else { "-".to_string() });
    println!("HostMax:   {}", if ipv4.prefix() < 31 { format!("{}", ipv4.nth(end_idx).expect("panic!")) } else { "-".to_string() });
    println!("Broadcast: {}", ipv4.broadcast());
    println!("Hosts/Net: {}", if ipv4.prefix() < 31 { end_idx - start_idx + 1 } else { 0 });
}

fn print_ipv4_enumerate(ipv4: &Ipv4Network, prefix: bool) {
    if ipv4.prefix() < 31 {
        let start_idx = get_host_min_idx(&ipv4);
        let end_idx = get_host_max_idx(&ipv4);

        for x in start_idx..=end_idx {
            match ipv4.nth(x) {
                None => {}
                Some(addr) => println!("{}/{}", addr, if prefix { format!("{}", ipv4.prefix()) } else { format!("{}", ipv4.mask())})
            }
        }
    }
}

fn main() {
    // Parse arguments.
    let args = Args::parse();

    let re_addr_v4_p1 = get_ipaddress_pattern1_regex();
    let re_addr_v4_p2 = get_ipaddress_pattern2_regex();

    let ipv4_result = match (re_addr_v4_p1.is_match(&args.address), re_addr_v4_p2.is_match(&args.address)) {
        (true, false) => {
            // Pattern 1 (xxx.xxx.xxx.xxx/xx)
            parse_ipaddress_pattern1(&args)
        },
        (false, true) => {
            // Pattern 2 (xxx.xxx.xxx.xxx/xxx.xxx.xxx.xxx)
            parse_ipaddress_pattern2(&args)
        },
        _ => {
            Err("Incorrect IP address/subnet mask (prefix).")
        }
    };
    match ipv4_result {
        Ok(result) => {
            if args.enumerate {
                print_ipv4_enumerate(&result, args.prefix)
            } else {
                print_ipv4_info(&result)
            }
        },
        Err(err) => {
            eprintln!("{}", err.to_string())
        }
    }
}
