# Smart Pointers & Interior Mutability :

we will understand **how Rust manages heap memory, shared ownership, and controlled mutation**, which are the foundation for production Rust systems like:

- Browsers
- Compilers
- Game Engines
- Graph Databases
- Async runtimes
- GUI frameworks

#### Topics :
- 📦 `Box<T>`
- 🔄 `Rc<T>`
- 🌍 `Arc<T>`
- 🎯 `Deref`
- 💀 `Drop`

#### Problem :
Sometimes we need something that
- is too large
- recursive
- shared
- dynamically sized
This is where smart pointers come in.

```rust
Box<T>
Rc<T>
Arc<T>
```

contains :
- pointer
- metadata
- safety logic
- automatic cleanup

---

> Starting off from Smart Pointers :
## 1. `Box<T>` :

>`Box<T>` means - *Put this value on the **Heap***.

```rust
fn main() {
    let x = Box::new(1000);
}
```

Memory :
```
Stack: x
Heap: 1000
```

### Why ?
> **1. Large Data**

Instead of :
```rust
struct Big {
	data: [u8; 1000000]
}
```

We will use :
```rust
struct Big {
	data: Box<[u8; 1000000]>
}
```
Now the struct stays small.

> **2. Recursive Types**

This is impossible :
```rust
struct Node {
	value: i32,
	next: Node,
}
```

Compiler will say : **Infinite Size**. But why?
Node Contains Node
Which Contains Node
Which Contains Node.... Infinte.

So to Solve this, we write the above code like :
```rust
struct Node {
	value: i32,
	next: Option<Box<Node>>,
}
```

Now :
- Node
- value
- pointer
Means Fixed Size.

- An Example code snippet will be like this : 

```rust
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let node3 = Node {
        value: 3,
        next: None,
    };
    let node2 = Node {
        value: 2,
        next: Some(Box::new(node3)),
    };
    let node1 = Node {
        value: 1,
        next: Some(Box::new(node2)),
    };
}
```

Internal Working :
```
-----Stack--------------------------------
node1
↓
Heap
Node
|
pointer
↓
Node
↓
Node
```

- in short : Every node owns the next node.
- Ownership chain.
- No Leaks.

---
## 2. Deref Trait :

>Deref trait is just `*` with trailing variable or const. i.e `*name`.

in the most simple term lest's take a code :
```rust
fn main() {
    let x = 5;
    let y = &x;
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- datatype of `x` is `i32`.
- we have taken `y = &x` which made the datatype of `y` as `&i32`.
- on `assert_eq!(5, x)` the datatype of `5` is `i32` and `x` was also `i32`.
- on `assert_eq!(5, *y)` the datatype of `5` is `i32` and `*y` made the datatype of `y` from `&i32` -> `i32` . which made this possible to compile correctly. of we don't use `*` here then the datatype mismatch will occur bcz after that the `i32 != &i32`. it must be same datatype in both sides.

### Your own Smart Pointer :

```rust
use std::ops::Deref;
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
	type Target=T;
	
	fn deref(&self) -> &T {
		&self.0
	}
}

fn main() {
	let x = MyBox(15);
	println!("{}", *x);
}
```

- Now your own smart Pointer behaves exactly like `Box`.

### Deref Coercion :

```rust
fn main() {
    let s = String::from("Rust");
    hello(&s);
}

fn hello(name: &str) {
    println!("{}", name);
}
```

- Why ??
- `hello()` expects `&str`. but you passed `&String`.
- Rust Automatically Performs :
	- `&String`.
	- `deref()`.
	- `&str`.
- No copy. No Allocation. Zero Cost.

---
## 3. Drop Trait :

- Rust Automatically destroys values.
- But you can customize cleanup.

```rust
#[derive(Debug)]
struct Database;

impl Drop for Database {
    fn drop(&mut self) {
        println!("Closing Connection...");
    }
}

fn main() {
    let db = Database;
    println!("{:?}", db);
}

// Output: Closing Connection...
```

Production uses :
- sockets
- file handles
- mutexes
- GPU buffers
Everything cleans automatically. This is RAII.

### Internal Timeline :

```
main starts.
Database created.
work...
main end.
drop().
memory freed.
```

- No garbage collector needed.

---
## 4. `Rc<T>` (Reference Counted Pointer) :

>Data lives until **all** of them die.
### The Problem :

Rust's rule: **one value, one owner.**

```rust
let a = String::from("Rust");
let b = a;  // a is dead. ownership moved.
```

Fine for linear data. Breaks for **graphs**:

```
      Root
    /     \
    A     B
    \     /
	   C        ← C belongs to BOTH A and B. Impossible with Box.
```

`Box` = exclusive ownership. You can't give C to both A and B.

### `Rc<T>` — Everyone shares, nobody exclusively owns :

```rust
use std::rc::Rc;

let a = Rc::new(String::from("Rust"));
let b = Rc::clone(&a);  // not a data copy — just +1 to counter
let c = Rc::clone(&a);  // +1 again
```

```
         ┌──────────┐
    a ──►│          │
         │ "Rust"   │
    b ──►│ count = 3│
         │          │
    c ──►└──────────┘
```

- One allocation on heap. Three pointers to it. Data lives until **all** of them die.

### `Rc::clone()` ≠ data copy :

|What copies|Memory|
|---|---|---|
|`s.clone()`|The actual data|2 allocations|
|`Rc::clone(&a)`|Just the pointer|1 allocation, counter++|
### Reference Count - Live Example :

```rust
let data = Rc::new(100);
println!("{}", Rc::strong_count(&data)); // 1

let a = Rc::clone(&data);
println!("{}", Rc::strong_count(&data)); // 2

{
    let b = Rc::clone(&data);
    println!("{}", Rc::strong_count(&data)); // 3
}   // b dropped here

println!("{}", Rc::strong_count(&data)); // 2
```

```
data created  →  count = 1
clone → a     →  count = 2
clone → b     →  count = 3
b drops       →  count = 2
              →  count = 0  →  memory freed ✅
```
### Real Uses :

- Shared config across modules
- Nodes in a graph/tree with multiple parents
- GUI widgets with multiple references
- Game assets (textures, meshes shared across entities)
### Hard Limit :

```
Rc<T>  ❌  NOT thread-safe
Arc<T> ✅  Thread-safe version (atomic counter)
```

If you `Rc` across threads → **compiler refuses to compile.** Not a runtime crash - a compile error. Rust catches it before it can hurt you.

---
## 5. `Arc<T>` - Atomic Reference Counted :

> Exactly like `Rc` except `Multiple Threads` are allowed.

```rust
use std::{sync::Arc, thread};

fn main() {
    let value = Arc::new(100);
    let mut handles = Vec::new();
    
    for _ in 0..5 {
        let num = Arc::clone(&value);
        
        handles.push(thread::spawn(move ||{
            println!("{}", num);
        }));
    }
    
    for h in handles {
        h.join().unwrap();
    }
}
```

### `Rc` vs `Arc` :

| Rc            | Arc                       |
| ------------- | ------------------------- |
| Single thread | Multi thread              |
| Faster        | Slightly slower           |
| No atomics    | Atomic reference counting |
### Golden Rule :

```
- One thread?
	`Rc<T>`

- Multiple threads?
	`Arc<T>`
```

---
# Interior Mutability :

#### Topics :
- 🔒 Interior Mutability
    - `Cell<T>`
    - `RefCell<T>`
    - `Rc<RefCell<T>>`

Normally :

```rust
fn main() {
    let x = 5;
    x = 10;
}
```

- Its Impossible.

```rust
fn main() {
    let x = vec![1, 2, 3];
    
    let a = &x;
    let b = &mut x;
}
```

- Compiler will never pass this because : `shared borrow + mutable borrow` is illegal.

>Sometimes we know : "This mutation is actually safe."
>Rust says use : `Interior Mutability` .

## 1. `Cell<T>` :

> For `Copy Types`.

```rust
use std::cell::Cell;

fn main() {
    let x = Cell::new(10);
    println!("{}", x.get());
    
    x.set(20);
    println!("{}", x.get());
}
```

- Notice the Above code and you will realize that `x` was never set to `mut`. but still we can influence its data.

- It works only for :
	- i32
	- bool
	- char
	- usize
	- ...

## 2. `RefCell<T>` :

> Much more powerful. bcz it allows `Runtime Borrow Checking` instead of compile-time.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(String::from("Rust"));
    data.borrow_mut().push_str(" Language");
    
    println!("{}", data.borrow());
}
```

- Again Notice that `data` is still not mutable here.

### Internal Difference :

Normal borrow :

```
Compile Time

✓ Safe
✓ Fast
✓ No Runtime Cost
```

RefCell :

```
Runtime

Borrow Counter
Mutable?
Shared?
```

- If violated : `panic!` instead of compile error.
- Example of `panic!` is below :

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(10);
    let a = value.borrow_mut();
    let b = value.borrow_mut();
    
    println!("{:?}", a);
    println!("{:?}", b);
}
```

- It will show this :

```bash
thread 'main' (228442) panicked at src/main.rs:6:19:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- Compiler allows it, Runtime catches it.

## 3. `Rc<RefCell<T>>` :

This combination appears **everywhere**. Why?

```
Rc
↓
Shared Ownership
+
RefCell
↓
Mutable Data
```

> Now everyone shares AND everyone can mutate.

```rust
use std::{cell::RefCell, rc::Rc};

fn main() {
    let value = Rc::new(RefCell::new(10));
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    
    *a.borrow_mut() += 5;
    *b.borrow_mut() += 20;
    
    println!("{}", value.borrow());
} // Output: 35.
```

- Memory :

```
      Rc
    /   \
   a     b
      │
      ▼
   RefCell
      │
	  ▼
	 35
```

### Production Uses :
#### GUI Libraries :

```
Window
 │
 ├── Button
 ├── Menu
 └── TextBox
```

- All widgets share application state.

#### AST Nodes

```
Expression
	↓
Identifier
	↓
 Literal
```

- `Parser, Optimizer, Type Checker` all share nodes.
#### Notion-like Editor :

```
Document
│
├── Heading
├── Paragraph
└── Table
```

- Multiple editor components need to access and update the same document tree.

---
## The Problem with `Rc` :

- Suppose :

```
  Parent
	│
	▼
  Child
	▲
	│
  Parent
```

- Both own each other.
- Reference count :

```
Parent = 1
Child = 1
```

- Even after :

```
drop(parent);
```

- counts never become `0` which causes **Memory Leak**.

##  `Weak<T>`

- does **not** increase the reference count.

- Typical tree :
```
Parent
  │
  ▼
Child
```

Implementation :
- Parent → Child : `Rc`
- Child → Parent : `Weak`

> This breaks the ownership cycle while still allowing the child to access its parent if it still exists.

## Summary Cheat Sheet :

| Smart Pointer    | Ownership  | Mutable              | Thread Safe        | Primary Use                           |
| ---------------- | ---------- | -------------------- | ------------------ | ------------------------------------- |
| `Box<T>`         | Single     | Yes <br>(owner only) | ✅                  | Recursive structures, heap allocation |
| `Rc<T>`          | Shared     | No                   | ❌                  | Shared read-only ownership            |
| `Arc<T>`         | Shared     | No                   | ✅                  | Shared ownership across threads       |
| `Cell<T>`        | Single     | Yes (`Copy` types)   | ❌                  | Interior mutability for simple values |
| `RefCell<T>`     | Single     | Yes                  | ❌                  | Runtime borrow checking               |
| `Rc<RefCell<T>>` | Shared     | Yes                  | ❌                  | Mutable shared state in one thread    |
| `Arc<Mutex<T>>`  | Shared     | Yes                  | ✅                  | Mutable shared state across threads   |
| `Weak<T>`        | Non-owning | N/A                  | Matches `Rc`/`Arc` | Break ownership cycles                |
