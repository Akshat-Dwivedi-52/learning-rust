use std::{collections::BTreeMap, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();

    for i in 1..=10 {
        let handle = tokio::spawn(async move {
            println!("Task: {} Started.", i);
            sleep(Duration::from_secs(1)).await;
            (i, i*i)
        });
        
        handles.push(handle);
    }

    let mut squares = BTreeMap::new();

    for handle in handles {
        let (key, value) = handle.await.unwrap();
        squares.insert(key, value);
    }

    println!("{:?}", squares);
}
