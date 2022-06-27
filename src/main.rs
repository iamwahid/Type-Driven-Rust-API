use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<It> {
    iter: It,
    i: usize,
    bound: Option<usize>,
}

// associate method with a Type
// Impl block is quantified: for all type It implement progress of It, below call is valid to Progress
impl<It> Progress<It> {
    pub fn new(iter: It) -> Self {
        Progress {
            iter,
            i: 0,
            bound: None,
        }
    }
}

//Bounded and Unbounded
impl<It> Progress<It>
where
    It: ExactSizeIterator,
{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
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
        print!("{}", CLEAR);
        match self.bound {
            Some(bound) => {
                println!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i));
            }
            None => {
                println!("{}", "*".repeat(self.i));
            }
        }
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

// implement Trait to Type Generic, 🤯🤯🤯
impl<It> ProgressIteratorExt for It {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn main() {
    // unbouded
    // for i in (0..).progress().with_bound() {
    //     expensive_calculation(&i);
    // }
    let v = vec![1, 2, 3, 4, 5];
    // Trait superpower start here, extending existing function without touching the function itself
    for i in v.iter().progress().with_bound() {
        expensive_calculation(i);
    }
}

fn expensive_calculation(_n: &i32) -> &i32 {
    sleep(Duration::from_secs(1));
    _n
}
