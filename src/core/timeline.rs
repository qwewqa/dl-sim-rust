use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::cell::{RefCell, Cell};
use std::rc::{Rc, Weak};
use std::any::Any;

type RcRefCellBox<T> = Rc<RefCell<Box<T>>>;

pub fn new_rc_ref_cell_box<T>(x: T) -> RcRefCellBox<T> {
    Rc::new(RefCell::new(Box::new(x)))
}

pub struct Timeline {
    time: Cell<f64>,
    queue: RefCell<BinaryHeap<Rc<Trigger>>>,
}

impl Timeline {
    pub fn new() -> Timeline {
        Timeline {
            time: Cell::new(0.0),
            queue: RefCell::new(BinaryHeap::new()),
        }
    }

    pub fn schedule<F: FnMut() -> () + 'static>(&self, delay: f64, action: F) -> Weak<Trigger> {
        let r = Rc::new(Trigger {
            time: self.time.get() + delay,
            action: RefCell::new(Box::new(action)),
            cancelled: Cell::new(false),
        });
        self.queue.borrow_mut().push(r.clone());
        Rc::downgrade(&r)
    }

    pub fn run(&self) -> Result<(), ()> {
        while let Some(next) = self.queue.borrow_mut().pop() {
            if next.cancelled.get() { continue; };
            self.time.set(next.time);
            (&mut *next.action.borrow_mut())();
        }
        Ok(())
    }

    pub fn now(&self) -> f64 {
        self.time.get()
    }
}

pub struct Trigger {
    time: f64,
    action: RefCell<Box<dyn FnMut()>>,
    cancelled: Cell<bool>,
}

impl Trigger {
    fn cancel(&self) {
        self.cancelled.set(true);
    }
}

impl Eq for Trigger {}

impl PartialEq for Trigger {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Ord for Trigger {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Trigger {
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
    fn schedule() {
        struct Context {
            timeline: Timeline,
            flag: Cell<bool>,
        }
        let c = Rc::new(Context {
            timeline: Timeline::new(),
            flag: Cell::new(false),
        });

        let tl = &c.timeline;
        let r = Rc::downgrade(&c.clone());
        let timing = 10.0;
        tl.schedule(timing, move || {
            let r = r.upgrade().unwrap();
            assert_eq!(r.timeline.now(), timing);
            r.flag.replace(true);
        });
        tl.run();
        assert!(c.flag.get());
    }

    #[test]
    fn cancellation() {
        struct Context {
            timeline: Timeline,
            flag: Cell<bool>,
        }
        let c = Rc::new(Context {
            timeline: Timeline::new(),
            flag: Cell::new(false),
        });

        let tl = &c.timeline;
        let r = Rc::downgrade(&c.clone());
        let timing = 10.0;
        let trigger = tl.schedule(timing, move || {
            r.upgrade().unwrap().flag.replace(true);
        });
        trigger.upgrade().unwrap().cancel();
        tl.run();
        assert!(!c.flag.get());
    }
}