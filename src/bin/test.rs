#[macro_use]
extern crate maplit;
extern crate correlation;

use correlation::{config, conditions, Correlator};
use std::thread;

fn main() {
    let contexts = vec!{
        config::Context::new(conditions::Builder::new(100).build()),
        config::Context::new(conditions::Builder::new(100).build()),
        config::Context::new(conditions::Builder::new(100).build()),
    };
    let mut correlator = Correlator::new(contexts);
    let msg1 = btreemap!{
        "uuid".to_string() => "1".to_string(),
    };
    let _ = correlator.push_message(msg1);
    thread::sleep_ms(2000);
    let _ = correlator.stop();
}
