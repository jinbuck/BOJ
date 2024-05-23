use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("jinbuck");
    let tokens: Vec<&str> = input_line.trim().split(" ").collect();
    let up = tokens[0].parse::<i32>().unwrap();
    let down = tokens[1].parse::<i32>().unwrap();
    let height = tokens[2].parse::<i32>().unwrap();
    let mut count = 1;
    count += (height - up) / (up - down);
    if (height - up) % (up - down) != 0 {
        count += 1;
    }
    println!("{}", count);
}
