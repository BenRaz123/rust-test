use text_io::read;

fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn prompt(prompt: &str) -> String {
    print!("{prompt}");
    return read!();
}

fn main() {
    
    // Taking IO in Rust

    let name = prompt("What is Your Name? ");
    greet(&name);

}
