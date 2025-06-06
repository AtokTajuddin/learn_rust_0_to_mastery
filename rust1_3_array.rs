fn main(){
    let mut angka_vector: Vec<i32> = Vec::new();
    angka_vector.push(10);
    angka_vector.push(20);
    angka_vector.push(30);
    angka_vector.push(40);
    angka_vector.push(100);

    println!("Isi dari array yang ada ialah {} {}", angka_vector[0], angka_vector[1]);
    println!("Jumlah banyaknya elemen {}",angka_vector.len());

    match angka_vector.get(2){
        Some(angka) => println!("Elemen kedua adalah {}", angka),
        None => println!("Tidak ada elemen kedua"),
    }

    println!("Isi dari array yang ada ialah {:#?}", angka_vector);
    println!("Isi dari array yang ada ialah {:?}", angka_vector);
    /*
    Jika menuliskan println alias mencetak dengan menggunakan arugumen {:#?} maka array akan dicetak secara ascending, 
    jika println! menggunakan argumen {:?} maka array dicetak normal.
     */


}