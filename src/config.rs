use std::io::{Error, ErrorKind};
use std::net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = env!("CARGO_PKG_NAME"), 
    rename_all = "kebab-case")]
struct Options {
    ///Allowed IP addresses
    #[structopt(short, long, parse(try_from_str = parse_ip_addr), value_name = "ip addr", default_value = "127.0.0.1 ::1")]
    allowed: Vec<IpAddr>,

    ///Enable peer blocklists
    #[structopt(short, long)]
    blocklist: bool,

    ///Disable peer blocklists
    #[structopt(short = "B", long)]
    no_blocklist: bool,

    ///Where to watch for new .torrent files
    #[structopt(short = "c", long, parse(from_os_str), value_name = "directory")]
    watch_dir: Option<PathBuf>,

    ///Disable the watch-dir
    #[structopt(short = "C", long)]
    no_watch_dir: bool,

    ///Where to store new torrents until they're complete
    #[structopt(long, parse(from_os_str), value_name = "directory")]
    incomplete_dir: Option<PathBuf>,

    ///Don't store incomplete torrents in a different location
    #[structopt(long)]
    no_incomplete_dir: bool,

    ///Dump the settings and exit
    #[structopt(short, long)]
    dump_settings: bool,

    ///Dump the log messages to this filename
    #[structopt(short = "e", long, parse(from_os_str), value_name = "filename")]
    logfile: Option<PathBuf>,

    ///Run in the foreground instead of daemonizing
    #[structopt(short, long)]
    foreground: bool,

    ///Where to look for configuration files
    #[structopt(short = "g", long, parse(try_from_str = check_dir_exists), value_name = "path")]
    config_dir: Option<PathBuf>,

    ///RPC port
    #[structopt(short, long, value_name = "port", default_value = "9091")]
    port: u16,

    ///Require authentication
    #[structopt(short = "t", long)]
    auth: bool,

    ///Don't require authentication
    #[structopt(short = "T", long)]
    no_auth: bool,

    ///Set username for authentication
    #[structopt(short, long)]
    username: Option<String>,

    ///Set password for authentication
    #[structopt(short = "v", long)]
    password: Option<String>,

    ///Show error messages
    #[structopt(long)]
    log_error: bool,

    ///Show error and info messages
    #[structopt(long)]
    log_info: bool,

    ///Show error, info, and debug messages
    #[structopt(long)]
    log_debug: bool,

    ///Where to save downloaded data
    #[structopt(short = "w", long, parse(from_os_str), value_name = "directory")]
    download_dir: Option<PathBuf>,

    ///Pause all torrents on startup
    #[structopt(long)]
    paused: bool,

    ///Enable distributed hash tables (DHT)
    #[structopt(short = "o", long)]
    dht: bool,

    ///Disable distributed hash tables (DHT)
    #[structopt(short = "O", long)]
    no_dht: bool,

    ///Enable local peer discovery (LPD)
    #[structopt(short = "y", long)]
    lpd: bool,

    ///Disable local peer discovery (LPD)
    #[structopt(short = "Y", long)]
    no_lpd: bool,

    ///Enable uTP for peer connections
    #[structopt(long)]
    utp: bool,

    ///Disable uTP for peer connections
    #[structopt(long)]
    no_utp: bool,

    ///RPC port
    #[structopt(short = "P", long, value_name = "port", default_value = "51413")]
    peerport: u16,

    ///Enable portmapping via NAT-PMP or UPnP
    #[structopt(short = "m", long)]
    portmap: bool,

    ///Disable portmapping
    #[structopt(short = "M", long)]
    no_portmap: bool,

    ///Maximum overall number of peers    
    #[structopt(short = "L", long, value_name = "limit", default_value = "200")]
    peerlimit_global: u16,

    ///Maximum number of peers per torrent    
    #[structopt(short = "l", long, value_name = "limit", default_value = "50")]
    peerlimit_torrent: u16,

    ///Encrypt all peer connections
    #[structopt(long)]
    encryption_required: bool,

    ///Prefer encrypted peer connections
    #[structopt(long)]
    encryption_preferred: bool,

    ///Prefer unencrypted peer connections
    #[structopt(long)]
    encryption_tolerated: bool,

    ///Where to listen for peer connections
    #[structopt(short = "i", long, parse(try_from_str = parse_ipv4_addr), value_name = "ipv4 addr")]
    bind_address_ipv4: Option<Ipv4Addr>,

    ///Where to listen for peer connections
    #[structopt(short = "I", long, parse(try_from_str = parse_ipv6_addr), value_name = "ipv6 addr")]
    bind_address_ipv6: Option<Ipv6Addr>,

    ///Where to listen for peer connections
    #[structopt(short = "r", long, parse(try_from_str = parse_ip_addr), value_name = "ip addr")]
    rpc_bind_address: Option<IpAddr>,

    ///All torrents, unless overridden by a per-torrent setting, should seed until a specific ratio
    #[structopt(long, value_name = "ratio")]
    global_seedratio: Option<String>,

    ///All torrents, unless overridden by a per-torrent setting, should seed regardless of ratio
    #[structopt(long)]
    no_global_seedratio: bool,

    ///Enable PID file    
    #[structopt(short = "x", long, value_name = "pid-file")]
    pid_file: Option<u32>,
}

fn parse_ip_addr(src: &str) -> Result<IpAddr, AddrParseError> {
    src.trim().parse::<IpAddr>()
}

fn parse_ipv4_addr(src: &str) -> Result<Ipv4Addr, AddrParseError> {
    src.trim().parse::<Ipv4Addr>()
}

fn parse_ipv6_addr(src: &str) -> Result<Ipv6Addr, AddrParseError> {
    src.trim().parse::<Ipv6Addr>()
}

fn check_dir_exists(src: &str) -> Result<PathBuf, std::io::Error> {
    use std::fs::metadata;
    let path = metadata(src)?;
    if path.is_dir() {
        Ok(Path::new(src).to_path_buf())
    } else {
        Err(Error::new(ErrorKind::Other, "Is not dir"))
    }
}

pub fn init_config() {
    Options::from_args();
}
