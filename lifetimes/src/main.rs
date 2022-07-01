use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct that contains a reference
// Internal variable and argument must have same lifetime to be valid
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // // Borrow checker
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x; // Assigning a reference that doesn't live past scope
    // }
    //
    // println!("r: {}", r); // r is dangling here

    // same code within same scope
    let x = 5;
    let r = &x;
    println!("r: {}", r);

    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    //
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    } // result variable becomes out of scope
      // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    let _s: &'static str = "I have a static lifetime.";
}
