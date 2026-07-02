#[macro_export]
macro_rules! server_log {
    ($($arg:tt)*) => {
        println!("[SERVER] {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! client_log {
    ($name:expr, $($arg:tt)*) => {
        println!("[CLIENT {}] {}", $name, format!($($arg)*));
    };
}