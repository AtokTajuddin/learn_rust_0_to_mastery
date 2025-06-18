fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error input yang anda masukkan salah");
    let contoh_tuple: Vec<String> = input
    .trim()
    .split_whitespace()
    .map(|s|s.parse().expect("Inputan salah"))
    .collect()
    println!("Nama: {}, Umur: {}", contoh_tuple[0], contoh_tuple[1]);
}