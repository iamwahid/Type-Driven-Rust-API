use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<It> {
    iter: It,
    i: usize,
}

// associate method with a Type
// Impl block is quantified: for all type It implement progress of It, below call is valid to Progress
impl<It> Progress<It> {
    pub fn new(iter: It) -> Self {
        Progress { iter, i: 0 }
    }
}

// implement Iterator Trait
// https://doc.rust-lang.org/std/iter/trait.Iterator.html
impl<It> Iterator for Progress<It>
where
    It: Iterator,
{
    type Item = It::Item;
    // implement print progress whenever next() called, usually on loop
    fn next(&mut self) -> Option<Self::Item> {
        // print progress
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

fn main() {
    let v = vec![1, 2, 3];
    for i in Progress::new(v.iter()) {
        expensive_calculation(i);
    }
}

fn expensive_calculation(_n: &i32) -> &i32 {
    sleep(Duration::from_secs(1));
    _n
}
