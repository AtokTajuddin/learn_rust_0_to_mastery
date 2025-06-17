// fn main(){
//     let spasi = "Ini adalah contoh string dengan spasi";
//     let spasi= spasi.len();
//     println!("Terdeteksi spasi {} ",spasi);
// }
fn hitung_spasi(text:&str) -> usize{
    text.chars().filter(|c| c.is_whitespace()).count()
}

/*
nama_variabel.chars().filter(|c|c.is_whitespace()).count()
*/

fn main(){
    let text = "Halo ini uji coba program fungsi pendeteksi spasi";
    let hitung_spasi_text = hitung_spasi(text);
    println!("Teks nya {} ",text);
    println!("Jumlah spasinya : {} ",hitung_spasi_text);
}
