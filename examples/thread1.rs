use anyhow::Result;
use rand::Rng;
use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建 Producers
    for i in 0..NUM_PRODUCERS {
        let tx: Sender<Msg> = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    // 创建 Consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: {:?}", msg);
        }
        Arc::new("cosumer result".to_string())
    });

    let c = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Consumer thread panicked: {:?}", e))?;

    println!("Consumer returned: {:?}", c);

    println!("Exiting main thread");
    Ok(())
}

fn producer(idx: usize, tx: Sender<Msg>) -> Result<(), anyhow::Error> {
    loop {
        let mut rng = rand::rng();
        let value = rng.random_range(0..100);
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rng.random_range(100..500);
        thread::sleep(Duration::from_millis(sleep_time));
        // random exit condition to prevent infinite loop
        if rng.random_range(0..10) == 0 {
            println!("Producer {} exiting", idx);
            break;
        }
    }
    Ok(())
}
