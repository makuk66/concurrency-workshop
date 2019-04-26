use crossbeam;
use std::{thread, time};

// This compiles and runs: cargo run --example 033_crossbeam_scope
//
// main thread running
// main thread continuing
// [examples/033_crossbeam_scope.rs:13] obj = Test {
//     n: 3
// }
// sub thread done
// main thread continuing more
// main thread done
//
// See https://docs.rs/crossbeam/*/crossbeam/fn.scope.html
// and also the example on https://docs.rs/crossbeam/0.3.0/crossbeam/struct.Scope.html#method.spawn

#[derive(Debug)]
struct Test {
    n: u32,
}

fn print(obj: &Test) {
    crossbeam::scope(|scope| {
        scope.spawn(move |_x| {
            thread::sleep(time::Duration::from_secs(1));
            dbg!(obj);
            thread::sleep(time::Duration::from_secs(1));
            println!("sub thread done");
        });
        println!("main thread continuing");
        // Note we don't even join explicitly here, though that works too.
    })
    .unwrap();
    // we don't get here until the sub thread is done
    println!("main thread continuing more");
}

fn main() {
    println!("main thread running");
    let obj = Test { n: 3 };
    print(&obj);
    println!("main thread done");
}
