fn main() {
    let args = std::env::args().skip(1).collect::<String>();
    match kalc_kman::kalc_f64(&args) {
        Ok(res) => {
            println!("{res}")
        }
        Err(error) => {
            eprintln!("Error: {error}")
        }
    }
}
