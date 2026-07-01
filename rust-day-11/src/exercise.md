> **Exercise-1** : Create an async function `async fn say_hi()` print `Hi`. call is correctly from an async `main`.

> **Exercise-2** : Create three async tasks that.
> 	- Sleep for different durations.
> 	- Print when they finish

> **Exercise-3** : Spawn 10 concurrent tasks :
> 	- Each task prints : `Task X Started.`
> 	- Wait `1 Sec` asynchronously, then prints : `Task X Finished.`
> 	- Finally await all `JoinHandle`s before exiting.

> **Exercise-4** : Write a program that:
> 	- Spawns **10 async tasks**.
> 	- Each task waits **1 second**.
> 	- Each task returns: `i * i`.
> 	- Store all the `JoinHandle`s in a vector.
> 	- Await every handle.
> 	- Collect the returned values into another vector.
> 	- Finally Print them.
> **Constraint:** Do **not** use any shared state (`Arc`, `Mutex`, etc.). Rely solely on each task's return value via `JoinHandle<T>`.

---
## Exercise-1 :

```rust
async fn say_hi() {
    println!("Hi");
}

#[tokio::main]
async fn main() {
    say_hi().await;
}
```

## Exercise-2 :

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {

    let h1 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task-1")        
    });
    let h2 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task-2")        
    });
    let h3 = tokio::spawn(async {
        sleep(Duration::from_secs(3)).await;
        println!("Task-3")        
    });
    
    h1.await.unwrap();
    h2.await.unwrap();
    h3.await.unwrap();
}
```

## Exercise-3 :

```rust
use std::{time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    
    for i in 1..=10 {
        let handle = tokio::spawn(async move {
            println!("Task: {} Started.", i);            
            sleep(Duration::from_secs(1)).await;
            println!("Task: {} Finished.", i);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
}
```

## Exercise-4 :

```rust
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
```

---
