fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len() // s is borrowed and cannot be modified
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    let s4 = String::from("hello");

    println!("{:?}, {:?}", calculate_length(s1), calculate_length(s3));
    println!("The length of '{}' is {}.", s4, calculate_length_ref(&s4));

    // Takes a mutable reference and changes the value
    let mut s5 = String::from("hello");
    change(&mut s5);
    println!("After changing: {}", s5);
    println!("After slicing: {}", first_word(&s5));

    // Also works for slicing arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
