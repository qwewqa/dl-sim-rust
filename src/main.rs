mod core;

fn main() {
    let mut tl = core::Timeline::new(0.0);
    tl.schedule(5.0);
    tl.schedule(8.0);
    tl.schedule(4.0);
    tl.show();
    tl.next();
    tl.show();
    tl.next();
    tl.show();
}
