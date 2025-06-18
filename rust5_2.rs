use std::io;

fn main(){
    let mut arr = String::new(); // Ini untuk proses pembuatan variabel input
    io::stdin().read_line(&mut arr).expect("Error membaca input");
    let angka : Vec<i32> = arr
    .trim()
    .split_whitespace() // Untuk memisahkan inputan berdasarkan spasi
    .map(|s|s.parse().expect("Input bukan angka")) // Proses konversi input ke integer
    .collect();
    println!("Array yang dimasukkan {:?} ",angka );
    println!("Jumlah elemen dalam array {} ",angka.len());
}