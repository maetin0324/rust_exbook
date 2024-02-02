fn main() {
    let u = "-4".parse::<u16>();
    match u  {
        Err(e) => eprintln!("parse error: {}", e),
        Ok(u) => println!("parsed into {}", u.to_string()),
    }
}
