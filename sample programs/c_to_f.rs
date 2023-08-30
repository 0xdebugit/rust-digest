fn main() {
    let f = c_to_f(20.3);
    println!("{}", f);
}

fn c_to_f(c: f64) -> f64 {
    (1.8 * c) + 32.0
    
}