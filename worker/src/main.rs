mod metrics;
mod monitor;

use eyre::Result;
use std::{thread::sleep, time::Duration};

use crate::metrics::MetricsReport;
use crate::monitor::Monitor;

// The value 500 is for quick visualization purposes, 
// in production there will have a reasonable value 
const TIME_STAMP_IN_MILLIS: u64 = 500; 

#[tokio::main]
async fn main()-> Result<()>{
    let mut monitor = Monitor::new("http://localhost:8080/http/agt_mgr")?;
    let mut metrics_report: MetricsReport = MetricsReport::new();

    loop{
        let metric = metrics_report.get_metrics();
        monitor.send_request(&metric).await?;
        println!("{metric:?}");
        sleep(Duration::from_millis(TIME_STAMP_IN_MILLIS));
    }

}
