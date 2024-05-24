use std::io;

fn main() {
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("jinbuck");
        let tokens: Vec<&str> = input_line.trim().split(" ").collect();
        let left = tokens[0].parse::<u32>().unwrap();
        let right = tokens[1].parse::<u32>().unwrap();
        if left == 0 && right == 0 {
            break;
        }
        let output = if left > right { "Yes" } else { "No" };
        println!("{}", output);
    }
}
