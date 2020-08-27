use serde::{Deserialize, Serialize};
//use serde_json::Result;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Serialize, Deserialize)]
enum EncryptionMode {
    ClearPrefered,
    EncryptionPrefered,
    EncryptionRequired,
}

#[derive(Serialize, Deserialize)]
enum LogLevel {
    LogError,
    LogInfo,
    LogDebug,
    LogFirehose,
}

#[derive(Serialize, Deserialize)]
enum PreallocationMode {
    PreallocationNone,
    PreallocationSparse,
    PreallocationFull,
}

#[derive(Serialize, Deserialize)]
struct SessionSettings {
    blocklist_enabled: bool,
    blocklist_url: Url,
    cache_size_mb: u16,
    dht_enabled: bool,
    utp_enabled: bool,
    ltp_enabled: bool,
    download_dir: Option<PathBuf>,
    speed_limit_down: u16,
    speed_limit_down_enabled: bool,
    encryption: EncryptionMode,
    idle_seeding_limit: u16,
    idle_seeding_limit_enabled: bool,
    incomplete_dir: Option<PathBuf>,
    incomplete_dir_enabled: bool,
    message_level: LogLevel,
    download_queue_size: u32,
    download_queue_enabled: bool,
    peer_limit_global: u16,
    peer_limit_per_torrent: u16,
    peer_port: u16,
    peer_port_random_on_start: bool,
    peer_port_random_low: u16,
    peer_port_random_high: u16,
    peer_socket_tos: String,
    pex_enabled: bool,
    port_forwarding_enabled: bool,
    preallocation: PreallocationMode,
    prefetch_enabled: bool,
    peer_id_ttl_hours: u16,
    queue_stalled_enabled: bool,
    queue_stalled_minutes: u16,
    ratio_limit: f32,
    ratio_limit_enabled: bool,
    rename_partial_files: bool,
    rpc_authentication_required: bool,
    rpc_bind_enabled_address: IpAddr,
    rpc_enabled: bool,
    rpc_password: String,
    rpc_username: String,
    rpc_whitelist: Vec<IpAddr>,
    rpc_whitelist_enabled: bool,
    rpc_host_whitelist: Vec<String>,
    rpc_host_whitelist_enabled: bool,
    rpc_port: u16,
    rpc_url: String,
    scrape_paused_torrents_enabled: bool,
    script_torrent_done_filename: String,
    script_torrent_done_enabled: bool,
    seed_queue_size: u16,
    seed_queue_enabled: bool,
    alt_speed_enabled: bool,
    alt_speed_up: u16,
    alt_speed_down: u16,
    alt_speed_time_begin: u16,
    alt_speed_time_enabled: bool,
    alt_speed_time_end: u16,
    alt_speed_time_day: u16,
    speed_limit_up: u16,
    speed_limit_up_enabled: bool,
    umask: u16,
    upload_slots_per_torrent: u16,
    bind_address_ipv4: Ipv4Addr,
    bind_address_ipv6: Ipv6Addr,
    start_added_torrents: bool,
    trash_original_torrent_files: bool,
}

impl Default for SessionSettings {
    fn default() -> SessionSettings {
        SessionSettings {
            blocklist_enabled: true,
            blocklist_url: Url::parse("http://www.example.com/blocklist").unwrap(),
            cache_size_mb: 4,
            dht_enabled: true,
            utp_enabled: true,
            ltp_enabled: false,
            download_dir: get_default_config_dir(),
            speed_limit_down: 100,
            speed_limit_down_enabled: false,
            encryption: EncryptionMode::EncryptionPrefered,
            idle_seeding_limit: 30,
            idle_seeding_limit_enabled: false,
            incomplete_dir: get_default_download_dir(),
            incomplete_dir_enabled: false,
            message_level: LogLevel::LogInfo,
            download_queue_size: 5,
            download_queue_enabled: true,
            peer_limit_global: 200,
            peer_limit_per_torrent: 50,
            peer_port: 51413,
            peer_port_random_on_start: false,
            peer_port_random_low: 49152,
            peer_port_random_high: 65535,
            peer_socket_tos: String::from("default"),
            pex_enabled: true,
            port_forwarding_enabled: true,
            preallocation: PreallocationMode::PreallocationSparse,
            prefetch_enabled: true,
            peer_id_ttl_hours: 6,
            queue_stalled_enabled: true,
            queue_stalled_minutes: 30,
            ratio_limit: 2.0,
            ratio_limit_enabled: false,
            rename_partial_files: true,
            rpc_authentication_required: false,
            rpc_bind_enabled_address: "0.0.0.0".parse().unwrap(),
            rpc_enabled: false,
            rpc_password: String::new(),
            rpc_username: String::new(),
            rpc_whitelist: vec!["127.0.0.1".parse().unwrap(), "::1".parse().unwrap()],
            rpc_whitelist_enabled: true,
            rpc_host_whitelist: vec![String::new()],
            rpc_host_whitelist_enabled: true,
            rpc_port: 9091,
            rpc_url: String::new(),
            scrape_paused_torrents_enabled: true,
            script_torrent_done_filename: String::new(),
            script_torrent_done_enabled: false,
            seed_queue_size: 10,
            seed_queue_enabled: false,
            alt_speed_enabled: false,
            alt_speed_up: 50,
            alt_speed_down: 50,
            alt_speed_time_begin: 540,
            alt_speed_time_enabled: false,
            alt_speed_time_end: 1020,
            alt_speed_time_day: 0,
            speed_limit_up: 100,
            speed_limit_up_enabled: false,
            umask: 022,
            upload_slots_per_torrent: 14,
            bind_address_ipv4: "0.0.0.0".parse().unwrap(),
            bind_address_ipv6: "::".parse().unwrap(),
            start_added_torrents: true,
            trash_original_torrent_files: false,
        }
    }
}

fn get_default_config_dir() -> Option<PathBuf> {
    use std::env;

    if let Ok(config_dir) = env::var("TRANSMISSION_HOME") {
        Some(Path::new(&config_dir).to_path_buf())
    } else if let Some(config_dir) = dirs::config_dir() {
        Some(config_dir.join(env!("CARGO_PKG_NAME")))
    } else if let Some(config_dir) = dirs::home_dir() {
        Some(config_dir.join(".config").join(env!("CARGO_PKG_NAME")))
    } else {
        None
    }
}

fn get_default_download_dir() -> Option<PathBuf> {
    dirs::download_dir()
}
