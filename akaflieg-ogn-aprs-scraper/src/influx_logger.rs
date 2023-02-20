use std::sync::Arc;

use log::LevelFilter;
use log::{Level, Metadata, Record};

use influxdb2::models::DataPoint;
use influxdb2::Client;

pub struct InfluxLogger {
    client: Arc<Client>,
    log_bucket: &'static str,
}

impl InfluxLogger {
    pub fn init(client: Arc<Client>, log_bucket: &'static str) {
        let logger = InfluxLogger { client, log_bucket };
        log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(LevelFilter::Info)).unwrap();
    }
}

impl log::Log for InfluxLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // write the log message to stdout
            println!("{} :: {}", record.level(), record.args());

            // write the log to influx
            let _type = record.level().as_str();
            let _content = record.args().to_string();
            tokio::spawn(log_to_influx(self.client.clone(), self.log_bucket, _type, _content));
        }
    }

    fn flush(&self) {}
}

async fn log_to_influx(
    client: Arc<Client>,
    log_bucket: &'static str,
    log_type: &'static str,
    log_content: String,
) {
    let datapoint = match DataPoint::builder("log")
        .tag("type", log_type)
        .field("content", log_content)
        .build()
    {
        Ok(p) => p,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match client
        .write(log_bucket, tokio_stream::once(datapoint))
        .await
    {
        Ok(p) => p,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };
}
