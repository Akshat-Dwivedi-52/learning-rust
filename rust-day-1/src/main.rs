fn main() {
    let s1 = String::from("akshat");

    let rev_output = reverse(&s1);
    println!("Reversed Output: {}", rev_output);

    let first_caps_only = capitalize(&s1);
    println!("First Caps Output: {}", first_caps_only);

    let substring = substring(&s1, 1, 4);
    println!("Substring: {}", substring);

    let count_words = word_count(&s1);
    println!("Word Counts: {}", count_words);
}

// Reversing the Text :
fn reverse(text: &str) -> String {
    let mut answer = String::from("");

    for char in text.chars().rev() {
        answer.push(char);
    }

    answer
}

// Making First Character Caps only :
fn capitalize(text: &str) -> String {
    let mut answer = String::from("");

    for char in text.to_uppercase().chars().into_iter().take(1) {
        answer.push(char);
    }

    for restletters in text.chars().into_iter().skip(1) {
        answer.push(restletters);
    }

    answer
}

// Getting the Substring :
fn substring(text: &str, start: usize, end: usize) -> String {
    let mut answer = String::from("");

    for char in text.chars().into_iter().skip(start).take(end) {
        answer.push(char);
    }

    answer
}

// Word Counting :
fn word_count(text: &str) -> usize {
    let counter = text.split_whitespace().count();
    counter
}
