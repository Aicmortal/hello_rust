fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    let word = first_word(&s);
    println!("{word}");

    let u1 = User {
        username: String::from("admin"),
        email: String::from("admin@localhost"),
        sign_in_count: 1,
        active: true,
    };

    println!("{:?}", u1);
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

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
