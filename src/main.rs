struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is short for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Function - it doesn't contain &self, often used as constructor
    // To use it, we call "Rectangle::square(10)" instead of "Rectangle.square(10)"
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

// Tuple struct is used when we want to name a tuple and differentiate from other tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Struct is useful when we want to implement a trait but don't have any data in the struct
struct AlwaysEqual;


fn using_struct() {
    println!("Using struct...");
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("some@email.com"),
        sign_in_count: 1,
    };

    println!("User1 {}", user1.username);
}

fn build_user(username: String, email: String) -> User {
    // We can use shorthand assignment if field key and a variable is identical
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn using_build_fn() {
    println!("Using build function...");
    let user = build_user(String::from("username"), String::from("email@email.com"));
    println!("Built user {}", user.username);
}

fn struct_update_syntax() {
    println!("Struct update syntax...");
    let user1 = build_user(String::from("user1"), String::from("user1@gmail.com"));
    let user2 = User {
        username: String::from("user2"),
        ..user1
    };
    println!("User2 email: {}", user2.email);
    // We can no loner user "user1.email" as its ownership is transferred to user2
    // println!("User1 {}", user1.email);
    // However, we can still use "user1.username" because its ownership is not transferred yet
    println!("User1 name: {}", user1.username);
    // Also, the simple data types like u64 and bool are copied (not moved). So, they're still valid
    println!("User1 active: {}", user1.active);
    println!("User1 sign in count: {}", user1.sign_in_count);
}

fn print_color_code(color: &Color) {
    println!("Color code {},{},{}", color.0, color.1, color.2);
}

fn using_tuple_struct() {
    println!("Using tuple structs...");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color r {}", black.0);
    println!("Origin x {}", origin.0);

    // Rust doesn't allow passing different tuple struct even if all field types are identical
    print_color_code(&black);
    // This line will cause a mismatch type error
    // print_color_code(&origin);
}

fn using_unit_like_struct() {
    println!("Using unit-like struct...");
    let subject = AlwaysEqual;
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn example_struct_program() {
    println!("Example_struct_program...");
    let rect1 = Rectangle {
        width: 32,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels", area(&rect1));
}

fn using_debug_print() {
    println!("Using debug print...");
    // Debug print is useful for developers as we won't need to implement formatting by ourselves
    // However, the format might not look good for users. In production, we should use std::fmt::Display trait instead
    let rect = Rectangle {
        width: 32,
        height: 50
    };

    // This :? printing requires the struct to #[derive(Debug)]
    println!("Debug rect single line {rect:?}");
    println!("Debug rect multiple lines {rect:#?}");
}

fn using_dbg_expression() {
    println!("Using dbg expression...");
    // dbg! prints out value put inside it
    // It is also an expression that returns the ownership of output it produces
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // It takes 30 * 2 = 60 inside, print it, and return the ownership back
        height: 50
    };

    dbg!(&rect1);
}

fn using_method() {
    println!("Using method...");
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };
    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect.can_hold(&rect3));
}

fn using_associated_functions() {
    println!("Using associated functions...");
    let square = Rectangle::square(10);

    println!("The area of the square is {} square pixels.", square.area());
}

fn main() {
    using_struct();
    using_build_fn();
    struct_update_syntax();
    using_tuple_struct();
    using_tuple_struct();
    using_unit_like_struct();
    example_struct_program();
    using_debug_print();
    using_dbg_expression();
    using_method();
    using_associated_functions();
}
