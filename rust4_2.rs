fn main(){
    let s:&str = "Halo ini contoh str";
    let i:&i32 = &100;
    println!("Ini contoh {} ",s);
    println!("Contoh int 32 {} ",i);
}

/*
Notes
Pastinya kita akan bingung dengan kode di atas terkait hal berikut :

mengapa dalam rust ini, 
konsepnya agak membingungkan ya, 
jika kita mencoba menuliskan datatype dengna let s:&str itu tidak perlu menggunakan shared borrow & pada penulisan isi variabelnya , 
berbanding beda dengan misal let i:&i32 kita harus dekralasi shared borrow lagi dalam isi variabelnya, mengapa ya

itu dikarenakan string atau &str disimpan pada static memory program
sedangkan seperti integer float dsj merupakan immediate value yang artinya
butuh tempat penyimpanan sebelum program dijalankan.
*/