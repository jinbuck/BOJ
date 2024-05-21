use std::io;
fn main() {
    let mut content = String::new();
    io::stdin().read_line(&mut content).expect("jinbuck");
    let content = content.trim();

    let ret = match content {
        "1 2 3 4 5 6 7 8" => "ascending",
        "8 7 6 5 4 3 2 1" => "descending",
        _ => "mixed",
    };

    println!("{}", ret);
}
