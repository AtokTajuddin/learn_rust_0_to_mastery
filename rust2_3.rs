// Belajar fungsi dan parameter
fn main(){
 sapa("Atok");
 let hasil_tambah = tambah(5,7);
 println!("Hasil penjumlahan 5 + 7 = {}", hasil_tambah);
println!("Luas lingkaran dengan jari-jari 5.5 adalah {}",hitung_luaslingkaran(5.5));
}

fn sapa(nama:&str){
    println!("Halo, {}!", nama);
}

fn tambah(angka1:i32, angka2:i32) -> i32 {
    angka1 + angka2
}
fn hitung_luaslingkaran(jari_jari:f64) -> f64{
    const PI:f64 = 3.14; //Deklarasi variabel sehingga harus ada ;
    PI*jari_jari*jari_jari
}