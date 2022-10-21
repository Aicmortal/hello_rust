fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    let lens = calc_length(&s);
    println!("{s}");
    println!("{lens}");
    change(&mut s);
    let lens = calc_length(&s);
    println!("{s}");
    println!("{lens}");
}

fn calc_length(s: &String) -> usize {
    s.len()
}

/// # chang
fn change(some_string: &mut String) {
    *some_string = String::from("123");
}
