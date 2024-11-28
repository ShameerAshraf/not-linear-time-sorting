// sort only positive numbers
// linear sort optimized for performance by halving sleep time

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::convert::TryInto;

fn main() {

    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9 , 10];

    linear_sort(array);

}

fn linear_sort(array: [i32; 10]) {

    let (tx, rx) = mpsc::channel();

    println!("hello");
    println!("{}", array[0]);

    thread::spawn(move || {

        for val in array {
            thread::sleep(Duration::from_millis(500 *  <i32 as TryInto<u64>>::try_into(val).unwrap()));
            tx.send(val).unwrap();
        }

    });

    for received in rx {
        //val = rx.recv().unwrap();
        println!("Got: {received}");
    }

}