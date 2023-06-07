fn main() {
    let args = std::env::args().skip(1).collect::<String>();
    println!("Res: {}", kalc::kalc_f64(&args).unwrap())
}
