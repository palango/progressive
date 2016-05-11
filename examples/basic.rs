extern crate progressive;

use progressive::progress;
use std::time::Duration;

fn main() {
    for _ in progress(0..30) {
        // do something expensive here
        std::thread::sleep(Duration::from_millis(200));
    }
}
