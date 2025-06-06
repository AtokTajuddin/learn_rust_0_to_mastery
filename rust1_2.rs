/*
Belajar casting datatype dalam rust
*/

fn main(){
    let a: i32 = 10;
    let b: f64 = 22.5;
    let result = a as f64 +b;
    println!("Hasil penjumlahan: {}", result);
}