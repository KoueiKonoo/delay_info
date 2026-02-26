fn main() {
    if let Err(e) = delay_info::run() {
        eprintln!("Error: {}", e);
    }
}
