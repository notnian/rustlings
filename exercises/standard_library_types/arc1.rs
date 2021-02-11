// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and create an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;
use std::time::SystemTime;

fn main() {
    let numbers: Vec<_> = (0..30u32).collect();

    // println!("{:?}", numbers);

    let shared_numbers = Arc::new(numbers); // TODO
    let mut join_handles = Vec::new();

    println!("Main thread => {:?}", SystemTime::now());

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        join_handles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 9;
            }
            println!("Other thread => {:?}", SystemTime::now());
            // println!("Sum of offset {} is {}", offset, sum);
        }));
    }

    println!("Main thread => {:?}", SystemTime::now());

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }

    println!("Main thread => {:?}", SystemTime::now());
}
