use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn _read_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    // Read to string directly with selective error propagation using ?
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // panic! intended for unrecoverable errors
    // panic!("crash and burn");

    // out of bounds
    // let v = vec![1, 2, 3];
    // v[99];

    // recoverable errors using Result
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // let f = File::open("hello.txt");
    // Using match
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    // Using unwrap. Unwrap gives us the result or automatically calls the panic! macro
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                // Create if doesn't exist
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
