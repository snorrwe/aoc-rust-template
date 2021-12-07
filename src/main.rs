fn main() {
    let mut buffer = String::with_capacity(1024);
    while let Ok(size) = std::io::stdin().read_line(&mut buffer) {
        let line: &str = buffer.as_str().trim_end();
        if line.len() == 0 {
            break;
        }
        println!("line: {:?}", line);
        buffer.clear();
    }
}
