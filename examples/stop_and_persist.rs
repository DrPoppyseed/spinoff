use spinoff::{Color, Spinner, Spinners};
use std::{thread::sleep, time::Duration};

fn main() {
    let sp = Spinner::new(Spinners::Arc, "Loading...", Color::Blue);
    sleep(Duration::from_secs(5));
    sp.stop_and_persist("🍕", "Pizza!");
}
