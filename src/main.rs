use std::io::stdin;

fn main() {
    println!("Hello, world!");

    loop {
        let mut input_line = String::new();
        stdin().read_line(&mut input_line);

        println!("{}", input_line);
    }
}
