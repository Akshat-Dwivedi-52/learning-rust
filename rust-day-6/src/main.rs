use rust_day_6::{queue::Queue, stack::Stack, traits::Printable};

fn main() {
    demo_stack();
    demo_queue();
}

fn demo_stack() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.print();
}

fn demo_queue() {
    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    queue.enqueue(40);
    queue.enqueue(50);
    queue.dequeue();
    queue.print();
}
