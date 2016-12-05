use std::env;
use std::fs::File; 
use std::io::Read;

fn main() {
    let haystack_name = env::args()
        .skip(1)
        .next()
        .expect("må ha fil");

    let needle : Vec<_> = "MARIO".bytes().collect();

    println!("looking for \"{:?}\" in {}", needle, haystack_name);

    let haystack : Vec<_> = File::open(haystack_name)
        .expect("kunke opne fil")
        .bytes()
        .map(|r| r.expect("i/o error eller noe"))
        .collect();

    for i in 0..haystack.len() - needle.len() {
        let straw = &haystack[i..i+needle.len()];
        if straw == &needle[..] {
            println!("FOUND! {:?} -> {}", straw, String::from_utf8_lossy(straw));
        }
    }
}
