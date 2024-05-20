use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("jinbuck");
    let n = n.trim().parse::<i32>().unwrap();

    for _ in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("jinbuck");
        let tokens: Vec<&str> = input_line.trim().split(" ").collect();
        let h = tokens[0].parse::<i32>().unwrap();
        let c = tokens[2].parse::<i32>().unwrap();

        if c <= h {
            println!("{}01", c);
        } else {
            if c % h == 0 {
                println!("{}{:02}", h, c / h);
            } else {
                println!("{}{:02}", c % h, c / h + 1);
            }
        }
    }
}
