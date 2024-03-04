// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    let mut char = get_char(data);

    string_uppercase(char);
}

// Should not take ownership
fn get_char(mut data: String) -> String {
    data.chars().last().unwrap().to_string()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    let data = data.to_uppercase();

    println!("{}", data);
}
