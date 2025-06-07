fn main (){
    println!("Selamat datang di Rust!");
}


#[test] // Atribute atau Annotation
fn hello_test(){
    let a:String = ("Halo dari Rust!, ini merupakan test function/unit test").to_string();
    assert_eq!(a, "Halo dari Rust!, ini merupakan test function/unit test");
    println!("{}", a);
}

/*
Contoh hasil unit test:

Cara runningnya selain menggunakan cargo test, rustc --test rust.rs && ./rust

running 1 test
test hello_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

*/