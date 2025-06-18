use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let contoh_tuple: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    println!("Nama: {}, Umur: {}", contoh_tuple[0], contoh_tuple[1]);
}