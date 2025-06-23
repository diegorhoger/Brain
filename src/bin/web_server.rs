use brain::start_web_server;
use clap::{Arg, Command};
use log::info;

#[tokio::main]
async fn main() -> brain::Result<()> {
    env_logger::init();
    
    let matches = Command::new("brain-web-server")
        .version("1.0.0")
        .about("Brain AI Web Interface Server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Port to run the web server on")
                .default_value("3030")
        )
        .get_matches();

    let port: u16 = matches
        .get_one::<String>("port")
        .unwrap()
        .parse()
        .expect("Invalid port number");

    info!("🧠 Starting Brain AI Web Server on port {}", port);
    
    start_web_server(port).await?;
    
    Ok(())
} 