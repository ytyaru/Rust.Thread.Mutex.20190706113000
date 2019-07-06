/*
 * Rustのスレッド（メモリ共有。Mutex、Arc）。
 * CreatedAt: 2019-07-06
 */
use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}
