//use daemonize::Daemonize;
mod config;
mod session;

fn main() {
    config::init_config();
    //Daemonize::new().start().unwrap();
}
