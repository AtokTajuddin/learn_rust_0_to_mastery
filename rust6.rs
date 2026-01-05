/*Recalling rust programming language basics
Learn about parallel execution
*/

use std::thread;
use std::time::Duration;

fn main(){
    let handle = thread::spawn(||{
        //Daerah dalam thread
        for i in 1..10{
            println!("Child thread(anak thread)hiungan ke -{}",i);
            thread::sleep(Duration::from_millis(200));
        }
    });
    for i in 1..=3{
        println!("Main thread(utama) hitungan ke -{}",i);
        thread::sleep(Duration::from_millis(200));
    }
    handle.join().unwrap();
    println!("Semua thread selesai.");
}