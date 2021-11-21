use std::thread::sleep;
use std::time::Duration;

mod interval;

use self::interval::Interval;
fn main() {
    let interval = Interval::from_millis(500);
    let duration = Duration::from_millis(100);

    let mut last = interval.get_counter();

    (1..51).for_each(|f| {
        let curr = interval.get_counter();

        if curr != last {
            last = curr;
            println!("Iteration number {}, counter is {}", f, curr);
        }

        sleep(duration)
    })
}
