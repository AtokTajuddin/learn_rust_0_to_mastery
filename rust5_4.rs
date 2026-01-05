/*
Latihan program sederhana menu coffe shop.
*/
use std::io;

fn main(){
println!("Selamat datang di toko kopi Rustacean!");


    for in 1..5={
        println!("Menu Kopi Rustacean:");
        println!("1. Espresso");
        println!("2. Latte");
        println!("3. Cappuccino");
        println!("4. Ice Americano");
        println!("5. Almon Latte");

        println!("Silahkan pilih kopi favorit anda (1-5):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
    }
    for choice in 1..5={
        match choice {

            1 => println!("Anda memilih Espresso,pilihan yang tepat untuk memulai semangat,satu tegukan penuh energi!"),
            2 => println!("Anda memilih Latte,nikmati kelembutan susu dalam setiap tegukan!"),
            3 => println!("Anda memilih Cappuccino,perpaduan sempurna antara kopi dengan foam yang lembut!"),
            4 => println!("Anda memilih Ice Americano,perpaduan kopi dengan es memang menyegarkan,selamat menikmati!"),
            5 => println!("Anda memilih Almon Latte,nikmati sensasi kacang almond yang kaya rasa!"),
            _ => {println!("Pilihan anda tidak ada di menu,silahkan pilih kembali!"); continue;
            }
            //Bagian dalam dari match choice
        }
        //Bagian luar for loop choice    
    }

    //Bagian luar for loop in

}