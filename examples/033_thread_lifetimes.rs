use std::thread;

// This does not compile: cargo run --example 033_thread_lifetimes
//
// error[E0621]: explicit lifetime required in the type of `obj`
//  --> examples/033_thread_lifetimes.rs:9:13
//   |
// 8 | fn print(obj: &Test) {
//   |               ----- help: add explicit lifetime `'static` to the type of `obj`: `&'static Test`
// 9 |     let t = thread::spawn(move || {
//   |             ^^^^^^^^^^^^^ lifetime `'static` required
//
// error: aborting due to previous error
//
// For more information about this error, try `rustc --explain E0621`.
// error: Could not compile `concurrency-workshop`.

#[derive(Debug)]
struct Test {
    n: u32,
}

fn print(obj: &Test) {
    let t = thread::spawn(move || {
        dbg!(&obj);
    });

    t.join().unwrap();
}

fn main() {
    let obj = Test { n: 3 };
    print(&obj);
}
