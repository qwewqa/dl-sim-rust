mod core;

fn main() {
    let mut tl = core::Timeline::new(0.0);
    tl.schedule(5.0, || println!("Hi 5"));
    tl.schedule(8.0,|| println!("Hi 8"));
    tl.schedule(4.0, || println!("Hi 4"));
    tl.show();
    tl.next();
    tl.show();
    tl.next();
    tl.show();
}
