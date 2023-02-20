use std::sync::Arc;
use std::time::Duration;
use std::error::Error;
use std::{io, str};

use log::{info, warn, error};

use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::sync::broadcast::{self, Sender};

use influxdb2::Client;

mod influx_logger;
mod influx;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // load environment variables from .env file.
    dotenv::dotenv()?;
    let url = dotenv::var("INFLUX_URL")?;
    let org = dotenv::var("INFLUX_ORG")?;
    let token = dotenv::var("INFLUX_TOKEN")?;

    let aprs_addr = dotenv::var("APRS_ADDR")?;
    let aprs_login_str = dotenv::var("APRS_LOGIN_STR")?;


    // create and init InfluxDB client, and setup the logger.
    let client = Arc::new(Client::new(url, org, token));
    influx_logger::InfluxLogger::init(client.clone(), "logs");

    // setup the return channel for APRS messages from the TCP stream;
    // write all arriving messages to influx.
    let (tx, _) = broadcast::channel::<String>(32);
    tokio::spawn(influx::write_aprs(client.clone(), "aprs", tx.clone()));

    // Main connection loop
    loop {
        // connect to APRS server until we get disconnected, then loop.
        let result = connect(&aprs_addr, &aprs_login_str, &tx).await;
        // see if we have any unusual errors apart from timeout disconnects.
        if let Err(err) = result {
            error!("{:?}", err);
        }
    }
}

// Connect to APRS server, log all APRS messages to Influx
async fn connect(addr: &str, login_str: &str, aprs_tx: &Sender<String>) -> Result<(), Box<dyn Error>> {

    // establish a TCP connection to the APRS server.
    let mut stream = TcpStream::connect(addr).await?;
    info!("connected to {:?}", addr);

    let login_str = format!("{}\r\n", login_str);
    let login_buf: &[u8] = login_str.as_bytes();

    // connection to the APRS server requires authenticating as an anonymous read-only user
    // the login string is passed as an environment variable.
    // see: http://wiki.glidernet.org/aprs-interaction-examples
    // use -1 as password for anonymous connection.
    write(&mut stream, login_buf).await?;

    // we've authenticated, now enter a reading loop until we get a timeout
    read(&stream, aprs_tx).await?;

    warn!("disconnected (server timeout or empty message)");

    Ok(())
}

// loop while reading all incoming APRS messages
// returns Ok(()) on timeout, as this is expected behaviour and should only cause a reconnect.
async fn read(stream: &TcpStream, aprs_tx: &Sender<String>) -> Result<(), Box<dyn Error>> {
    loop {
        // according to spec, we should assume a timeout and reconnect,
        // if we do not receive any new messages in one minute.
        timeout(Duration::from_secs(60), stream.readable()).await??;

        // use a large enough buffer to fit all expected APRS messages
        let mut buf = [0; 512];

        match stream.try_read(&mut buf) {
            // we've gotten an empty message or a timeout, reconnect.
            Ok(0) => break,
            // we've received a message of n characters, log it to InfluxDB
            Ok(n) => {
                let string_rep = util::format_for_display(&buf);
                info!("(read {}) {}", n, string_rep);
                aprs_tx.send(string_rep)?;
            }
            // we're not ready to read yet, wait another loop until we can read the next message.
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            // we've received another kind of error, return the error and reconnect.
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}

// write one message to the TCP socket.
async fn write(stream: &mut TcpStream, str: &[u8]) -> Result<(), Box<dyn Error>> {
    loop {
        stream.writable().await?;

        match stream.try_write(str) {
            // we've successfully written n characters to the socket.
            Ok(n) => {
                info!("(write {}) {}", n, util::format_for_display(str));
                break;
            }
            // we're not ready to write yet, try again on the next loop.
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            // we've gotten another kind of error, return it.
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}