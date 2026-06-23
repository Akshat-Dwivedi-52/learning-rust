use rust_day_4::search;

#[test]
fn finds_matches() {
    let content = "
hello
world
rust
hello rust";

    let result = search("hello", content);

    assert_eq!(vec!["hello", "hello rust"], result);
}

#[test]
fn no_matches() {
    let content = "
hello
world
rust";

    let result = search("java", content);

    assert_eq!(Vec::<&str>::new(), result);
}

#[test]
fn empty_content() {
    let result = search("hello", "");

    assert_eq!(Vec::<&str>::new(), result);
}

#[test]
fn empty_query() {
    let content = "
hello
world
rust";

    let result = search("", content);

    assert_eq!(vec!["hello", "world", "rust"], result);
}

#[test]
fn exact_match() {
    let content = "hello";

    let result = search("hello", content);

    assert_eq!(vec!["hello"], result);
}

#[test]
fn case_sensitive() {
    let content = "
Hello
hello
HELLO";

    let result = search("hello", content);

    assert_eq!(vec!["hello"], result);
}

#[test]
fn many_matches() {
    let content = "
hello
hello
hello";

    let result = search("hello", content);

    assert_eq!(vec!["hello", "hello", "hello"], result);
}

#[test]
fn partial_word_match() {
    let content = "
rust
rustacean
trust
crust";

    let result = search("rust", content);

    assert_eq!(vec!["rust", "rustacean", "trust", "crust"], result);
}

#[test]
fn query_with_spaces() {
    let content = "
hello rust
hello
rust";

    let result = search("hello rust", content);

    assert_eq!(vec!["hello rust"], result);
}

#[test]
fn special_characters() {
    let content = "
hello!
hello?
hello.";

    let result = search("!", content);

    assert_eq!(vec!["hello!"], result);
}
