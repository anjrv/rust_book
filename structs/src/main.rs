struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// Anonymous tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)] // Needed for debug print formatting
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
// Alternatively "internal" method implementation:
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("third@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // Alternative:
    let _user2 = User {
        email: String::from("third@example.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area is {}.", rect1.area());
    println!("Rectangle is {:?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("1 can hold 2? {}", rect1.can_hold(&rect2));
}
