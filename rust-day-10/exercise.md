
# Exercises :

> **Exercise-1** : Create **5 threads**, each printing its thread number, then wait for all of them using `join()`.

> **Exercise-2** : Use `Arc<Mutex<i32>>` to implement a shared counter incremented **1000 times** across **10 threads**. Verify the final value is exactly **1000**.

> **Exercise-3** : Build a program where **3 producer threads** send messages through an `mpsc` channel, and the main thread receives and prints every message.

> **Exercise-4** : Create an `Arc<RwLock<HashMap<String, i32>>>` that supports multiple concurrent readers and occasional writers.


---
### Exercise-1 :

> Create **5 threads**, each printing its thread number, then wait for all of them using `join()`.

```rust
use std::{thread};

fn main() {
    let mut handles = Vec::new();
    
    for i in 1..=5 {
        let handle = thread::spawn(move || {
            println!("Thread-{}", i);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---
### Exercise-2 :

> Use `Arc<Mutex<i32>>` to implement a shared counter incremented **1000 times** across **10 threads**. Verify the final value is exactly **1000**.

```rust
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
        
            loop {
                let mut counter = counter.lock().unwrap();
                
                if *counter >= 1000 {
                    break;
                }
                
                *counter += 1;
                println!("{}", counter);
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

---
### Exercise-3 :

> Build a program where **3 producer threads** send messages through an `mpsc` channel, and the main thread receives and prints every message.

```rust
use std::sync::{mpsc};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    
    for i in 1..=3 {
        let tx = tx.clone();
        
        let handle = thread::spawn(move || {
            let data = format!("Message from Thread {:?}", i);             
            tx.send(data).unwrap();
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    for _ in 0..3 {
        println!("Received: {}", rx.recv().unwrap());
    }
}
```

---
### Exercise-4 :

> Create an `Arc<RwLock<HashMap<String, i32>>>` that supports multiple concurrent readers and occasional writers.

```rust
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

fn main() {
    let users = Arc::new(RwLock::new(HashMap::<String, u32>::new()));
    let mut handles = Vec::new();
    
    // ==========================
    // Writer Thread
    // ==========================
    {
        let users = Arc::clone(&users);
        
        let writer_handle = thread::spawn(move || {
            println!("\n========== WRITER ==========");
            println!("WRITER: Waiting for write lock...");
            
            {
                let mut writer = users.write().unwrap();
                println!("WRITER: Write lock acquired.");
                
                writer.insert("Akshat Dwivedi".to_string(), 19);
                writer.insert("Nitin Chauhan".to_string(), 18);
                
                println!("WRITER: Data inserted.");
                println!("WRITER: Holding write lock for 3 seconds...\n");
                
                // Hold the write lock so readers have to wait
                thread::sleep(Duration::from_secs(3));
            } // <-- Write lock released here automatically
            
            println!("\nWRITER: Write lock released.\n");
        });
        
        handles.push(writer_handle);
    }
    
    // Give the writer a small head start
    thread::sleep(Duration::from_millis(100));
    
    // ==========================
    // Reader Threads
    // ==========================
    for i in 1..=3 {
        let users = Arc::clone(&users);
        
        let handle = thread::spawn(move || {
            println!("READER-{i}: Waiting for read lock...");
            let reader = users.read().unwrap();
            println!("READER-{i}: Read lock acquired.");
            
            for (name, age) in reader.iter() {
                println!("READER-{i}: {name} -> {age}");
            }
            
            println!("READER-{i}: Finished reading.\n");
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("========== PROGRAM FINISHED ==========");
}
```

---
