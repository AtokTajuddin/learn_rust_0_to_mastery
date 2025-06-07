//Belajar variabel & mutabilitas
fn main(){
    let pesan: String = ("Halo dari bahasa Rust").to_string();
    let mut pesan_mut : String = ("Halo bro,ini merupakan konsep mutable dalam Rust").to_string();

    println!("Pesan non-mutable: {}", pesan);
    println!("Pesan mutable,sebelum diubah : {}",pesan_mut);

    pesan_mut = ("Halo bray, ini merupakan konsep mutable dalam Rust, gimana gampang kan ?").to_string();
    println!("Pesan mutable, setelah diubah : {}",pesan_mut);
}