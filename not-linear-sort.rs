// sort only positive numbers

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
    
    for val in array {
        let tx1 = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(val.try_into().unwrap()));
            tx1.send(val).unwrap();
        });
    }

    drop(tx);
    let mut vec = Vec::new();

    for received in rx {
        println!("Got: {received}");
        vec.push(received);
    }

    println!("Final state: {:?}", vec);

}