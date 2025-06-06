/*
Belajar casting datatype dalam rust
*/

fn main(){
    let a: i32 = 10;
    let b: f64 = 22.5;
    let result = a as f64 +b;
    let string = String::from("Halo ");
    let s:String = "World".to_string();
    let gabungkan_string = string.clone() + &s;
    println!("Gabungan string {}",gabungkan_string);
    println!("Hasil penjumlahan: {}", result);
    println!("{}", string);


    // metode lain let gabungkan_string = &string + s.as_str(); 
}