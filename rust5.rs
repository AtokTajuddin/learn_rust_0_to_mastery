// Slicing string
fn main(){
    let s:&str = "Halo dunia dari pemrograman Rust!";
    let slice = &s[0..4];
    println!("Original string : {}", s);
    println!("Slicing string: {}", slice);
}