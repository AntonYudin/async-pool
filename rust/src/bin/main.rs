
use std::time::Duration;
use std::sync::mpsc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

use pool::pool::Pool;

struct Value {
    value: i32
}

fn main() {

    let index = AtomicI32::new(0i32);

    let pool = Pool::new(5, move || Ok(Value {
        value: { index.fetch_add(1, Ordering::Relaxed) }
    }));

    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let mut pool = pool.clone();
        let tx = tx.clone();

        thread::spawn(move || {
            println!("thread [{}] started", i);
            match pool.process(move |resource| {
                    println!("got resource [{}]", resource.get().value);
                    thread::sleep(Duration::from_secs(1));
                    if i == 1 {
                        println!("simulate an error by not writing to the channel");
                    } else if let Err(error) = tx.send(resource.get().value * 10) {
                        println!("error sending data to a channel: {}", error);
                    }
            }) {
                Ok(_result) => println!("process [{}] finished successfully", i),
                Err(error) => println!("process [{}] failed: {}", i, error)
            }
        });
    }

    drop(tx);

    for value in rx {
        println!("received value {}", value);
    }

    thread::sleep(Duration::from_secs(1));

}

