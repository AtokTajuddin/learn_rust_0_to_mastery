// //Belajar mengenai if-else statement
// fn main(){
//     let angka = 10;
//     if angka %2 == 0{
//         println!("Angka {}, adalah bilangan genap",angka);
//     }else{
//         println!("Angka {}, adalah bilangan ganjil",angka);
//     }
// }
// use std::io; // By default dengan menggunakan ini inputan user akan otomatis spacing ke new line
use std::io::Write; // Untuk menulis ke console
fn main(){
    println!("Silahkan masukkan angka : ");
    let mut input = String::new(); // Ini untuk proses pembuatan variabel input
    io::stdin().read_line(&mut input).expect("Error membaca input");
    let angka: i32 = input.trim().parse().expect("Input bukan angka"); // Proses konversi input ke integer
    //Penulisan untuk konversi dari datatype string itu .trim().parse.expect("Ini sebagai handling jika terjadi error")

    if angka % 2 ==0{
        println!("Angka {},adalah bilangan genap",angka);
    }
    else{
        println!("Angka {},adalah bilangan ganjil",angka);
    }
}