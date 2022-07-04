//! An example demonstrating some common features for json_env_logger
//! To enable low level logging levels set an env variable RUST_LOG. i.e. RUST_LOG=TRACE

use log::{debug, error, info, trace, warn};

fn main() {
    // Use builder if more configuration is required.
    json_env_logger2::init();
    let trace_str = "some trace str";
    trace!("I am a trace {trace_str}");
    debug!("I am a debug: {:?}", "Hey Mom, I am debugging!");
    info!("I am an info");
    warn!("I am a warning");
    error!("I am an error");
}
