use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

//state of progressbar
struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<It, Bound> {
    iter: It,
    i: usize,
    bar: char,
    bound: Bound,
}

trait ProgressDisplay: Sized {
    fn display<It>(&self, progress: &Progress<It, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<It>(&self, progress: &Progress<It, Self>) {
        println!("{}", format!("{}", progress.bar).repeat(progress.i));
    }
}

impl ProgressDisplay for Bounded {
    fn display<It>(&self, progress: &Progress<It, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            format!("{}", progress.bar).repeat(progress.i),
            " ".repeat(self.bound - progress.i),
            self.delims.1,
        );
    }
}

// associate method with a Type
// Impl block is quantified: for all type It implement progress of It, below call is valid to Progress
impl<It> Progress<It, Unbounded> {
    pub fn new(iter: It) -> Self {
        Progress {
            iter,
            i: 0,
            bar: '█',
            bound: Unbounded,
        }
    }

    pub fn with_bar(mut self, bar: char) -> Progress<It, Unbounded> {
        self.bar = bar;
        self
    }
}

//Bounded and Unbounded
impl<It> Progress<It, Unbounded>
where
    It: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<It, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bar: self.bar,
            bound,
        }
    }
}

impl<It> Progress<It, Bounded>
where
    It: ExactSizeIterator,
{
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

// implement Iterator Trait
// https://doc.rust-lang.org/std/iter/trait.Iterator.html
impl<It, Bound> Iterator for Progress<It, Bound>
where
    It: Iterator,
    Bound: ProgressDisplay,
{
    type Item = It::Item;
    // implement print progress whenever next() called, usually on loop
    fn next(&mut self) -> Option<Self::Item> {
        // print progress
        print!("{}", CLEAR);
        self.bound.display(&self);
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

// implement Trait to Type Generic, 🤯🤯🤯
impl<It> ProgressIteratorExt for It {
    fn progress(self) -> Progress<It, Unbounded> {
        Progress::new(self)
    }
}

fn main() {
    let brkt = ('|', '|');
    // unbouded
    // for i in (0..).progress().with_delims(brkt) {
    //     expensive_calculation(&i);
    // }
    let v = vec![1, 2, 3, 4, 5];
    // Trait superpower start here, extending existing function without touching the function itself
    for i in v
        .iter()
        .progress()
        .with_bar('*')
        .with_bound()
        .with_delims(brkt)
    {
        expensive_calculation(i);
    }
}

fn expensive_calculation(_n: &i32) -> &i32 {
    sleep(Duration::from_secs(1));
    _n
}
