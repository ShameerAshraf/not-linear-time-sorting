// sort positive/negative numbers

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::convert::TryInto;
use std::cmp;

fn main() {
    let array: [i32; 13] = [6, 4, 9, 2, 8, 1, 7, 5, 3 , 10, -2, -5, -7];
    linear_sort(array);
}

fn linear_sort(array: [i32; 13]) {

    let mut increment = 0;
    for val in array {
        increment = cmp::min(val, increment);
    }

    increment = i32::abs(increment);

    println!("increment: {}", increment);

    let array_vec: Vec<_> = array.to_vec().iter().map(|x| x + increment).collect();

    println!("{:?}", array_vec);

    let (tx, rx) = mpsc::channel();
    
    for val in array {
        let tx1 = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs((val + increment).try_into().unwrap()));
            tx1.send(val + increment).unwrap();
        });
    }

    drop(tx);
    let mut vec = Vec::new();

    for received in rx {
        println!("Got: {received}");
        vec.push(received - increment);
    }

    println!("Final state: {:?}", vec);

}