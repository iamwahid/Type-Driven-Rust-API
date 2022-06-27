use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn progress<It>(it: It, f: fn(It::Item) -> It::Item)
where
    It: Iterator, // constraint removed
{
    let mut s = 1; // state of progress
    for i in it {
        println!("{}{}", CLEAR, "*".repeat(s));
        s += 1;
        f(i);
    }
}

fn main() {
    let v = vec![1, 2, 3];
    progress(v.iter(), expensive_calculation);
}

fn expensive_calculation(_n: &i32) -> &i32 {
    sleep(Duration::from_secs(1));
    _n
}
