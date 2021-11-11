use enigo::{Enigo, Key, KeyboardControllable};
use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let now = Instant::now();
    let mut enigo = Enigo::new();
    let mut key_presses_count: u64 = 0;

    println!("created enigo.\nentering busy keypress loop.");

    loop {
        enigo.key_click(Key::Control);
        key_presses_count += 1;

        println!("elapsed time: {}s, # key cycles: {}", now.elapsed().as_secs(), key_presses_count);

        thread::sleep(Duration::from_secs(120));
    }
}
