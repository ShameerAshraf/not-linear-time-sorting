// sort only positive numbers
// linear sort optimized for performance by halving sleep time

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::convert::TryInto;

fn main() {

    let array: [i32; 10] = [6, 4, 9, 2, 8, 1, 7, 5, 3 , 10];

    linear_sort(array);

}

fn linear_sort(array: [i32; 10]) {

    let (tx, rx) = mpsc::channel();

    println!("hello");
    println!("{}", array[0]);

    for val in array {
        let tx1 = tx.clone();
        thread::spawn(move || {
                thread::sleep(Duration::from_millis(500 *  <i32 as TryInto<u64>>::try_into(val).unwrap()));
                tx1.send(val).unwrap();
        });
    }

    let _ = tx.send(0);

    let mut vec = Vec::new();

    for received in rx {
        //val = rx.recv().unwrap();
        println!("Got: {received}");
        vec.push(received);
    }

    println!("{:?}", vec);

}