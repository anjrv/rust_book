use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // This will panic if index is out of bounds
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There is no third element."),
    }

    println!("Iterate over vector:");
    for i in &v {
        println!("{}", i);
    }

    let mut v1 = vec![100, 32, 57];
    for i in &mut v1 {
        println!("Before: {}", i);
        *i += 50;
        println!("After: {}", i);
    }

    let val = "initial contents";
    let _s = val.to_string();
    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");

    // Simple concat
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // Format strings
    let s3 = String::from("tic");
    let s4 = String::from("tac");
    let s5 = String::from("toe");
    let s6 = format!("{}-{}-{}", s3, s4, s5);
    println!("{}", s6);

    // Substring replace
    let s7 = "Rust is boring!";
    println!("{}", s7.replace("boring!", "interesting!"));

    // std hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Insert overwrites, or_insert only inserts if key doesn't exist
    scores.insert(String::from("Blue"), 25);
    // This also returns the stored value
    scores.entry(String::from("Yellow")).or_insert(25);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
