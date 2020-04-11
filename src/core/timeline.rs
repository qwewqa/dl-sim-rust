use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::cell::{RefCell, Cell};
use std::rc::{Rc, Weak};
use std::any::Any;
use bumpalo::Bump;

pub struct Timeline<T> {
    time: f64,
    queue: BinaryHeap<Rc<Trigger<T>>>,
}

impl<T> Timeline<T> {
    pub fn new() -> Timeline<T> {
        Timeline {
            time: 0.0,
            queue: BinaryHeap::new(),
        }
    }

    pub fn schedule(&mut self, delay: f64, value: T) -> Rc<Trigger<T>> {
        self.schedule_abs(self.time + delay, value)
    }

    pub fn schedule_abs(&mut self, time: f64, value: T) -> Rc<Trigger<T>> {
        assert!(time >= self.time);
        let r = Rc::new(Trigger {
            time,
            value,
            cancelled: Cell::new(false),
        });
        self.queue.push(r.clone());
        r
    }
}

impl<T> Iterator for Timeline<T> {
    type Item = Rc<Trigger<T>>;

    fn next(&mut self) -> Option<Rc<Trigger<T>>> {
        while let Some(next) = self.queue.pop() {
            if next.cancelled.get() { continue; }
            self.time = next.time;
            return Some(next)
        }
        None
    }
}

pub struct Trigger<T> {
    time: f64,
    value: T,
    cancelled: Cell<bool>,
}

impl<T> Trigger<T> {
    pub fn cancel(&self) {
        self.cancelled.set(true);
    }
}

impl<T> Eq for Trigger<T> {}

impl<T> PartialEq for Trigger<T> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<T> Ord for Trigger<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> PartialOrd for Trigger<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{RefCell, Cell};
    use std::rc::Rc;
    use std::ops::Deref;
    use std::borrow::Borrow;

    #[test]
    fn schedule_single() {
        let mut tl = Timeline::new();
        tl.schedule(10.0, ());

        let n = tl.next().unwrap();
        assert_eq!(n.time, 10.0);
        assert_eq!(tl.time, 10.0);

        assert!(tl.next().is_none());
    }

    #[should_panic]
    #[test]
    fn negative_delay() {
        let mut tl = Timeline::new();
        tl.schedule(-1.0, ());
    }

    #[should_panic]
    #[test]
    fn schedule_in_past() {
        let mut tl = Timeline::new();
        tl.schedule(10.0, ());
        tl.next();
        tl.schedule_abs(9.0, ());
    }

    #[test]
    fn schedule_order() {
        let mut tl = Timeline::new();
        tl.schedule(3.0, 3.0);
        tl.schedule(1.0, 1.0);
        tl.schedule(2.0, 2.0);

        let n = tl.next().unwrap();
        assert_eq!(tl.time, 1.0);
        assert_eq!(n.time, 1.0);
        assert_eq!(n.value, 1.0);
        let n = tl.next().unwrap();
        assert_eq!(tl.time, 2.0);
        assert_eq!(n.time, 2.0);
        assert_eq!(n.value, 2.0);
        let n = tl.next().unwrap();
        assert_eq!(tl.time, 3.0);
        assert_eq!(n.time, 3.0);
        assert_eq!(n.value, 3.0);
        assert!(tl.next().is_none())
    }

    #[test]
    fn cancellation() {
        let mut tl = Timeline::new();
        tl.schedule(3.0, 3.0);
        tl.schedule(1.0, 1.0);
        tl.schedule(2.0, 2.0).cancel();

        let n = tl.next().unwrap();
        assert_eq!(tl.time, 1.0);
        assert_eq!(n.time, 1.0);
        assert_eq!(n.value, 1.0);
        let n = tl.next().unwrap();
        assert_eq!(tl.time, 3.0);
        assert_eq!(n.time, 3.0);
        assert_eq!(n.value, 3.0);
        assert!(tl.next().is_none())
    }
}