use std::cmp::Ordering;
use std::collections::BinaryHeap;
// type Callback = fn();

#[derive(PartialEq, Debug)]
struct Trigger {
    time: f64,
    // callback: Callback,
}

impl Eq for Trigger {} // why tho

impl PartialOrd for Trigger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

impl Ord for Trigger {
    fn cmp(&self, other: &Trigger) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Timeline {
    now: f64,                  // current time
    heap: BinaryHeap<Trigger>, // heap of Triggers
}

impl Timeline {
    pub fn new(time: f64) -> Timeline {
        Timeline {
            now: time,
            heap: BinaryHeap::new(),
        }
    }

    pub fn schedule(&mut self, time: f64) {
        self.heap.push(Trigger { time: time })
    }

    pub fn next(&mut self) {
        let next = self.heap.pop();
        println!("exec={:?}", next);
        match next {
            Some(n) => self.now = n.time,
            None => (),
        }
    }

    pub fn show(&self) {
        println!("now={:.4}s", self.now);
        println!("next={:?}", self.heap.peek());
    }
}
