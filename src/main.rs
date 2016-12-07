use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let haystack_name = env::args()
        .skip(1)
        .next()
        .expect("må ha fil");

    let needle = "MARIO".bytes().collect::<Vec<_>>();
    let needle = normalize(&needle);

    println!("looking for \"{:?}\" in {}", needle, haystack_name);

    let haystack = File::open(haystack_name)
        .expect("kunke opne fil")
        .bytes()
        .map(|r| r.expect("i/o error eller noe"))
        .collect::<Vec<_>>();

    for i in 0..haystack.len() - needle.len() {
        let straw = &haystack[i..i+needle.len()];
        let strawn = normalize(straw);
        if strawn == needle {
            println!("FOUND! {:?} -> {}", straw, String::from_utf8_lossy(straw));
        }
    }
}

fn normalize(input : &[u8]) -> Vec<i16> {
    let null = input[0] as i16;
    input.iter()
        .map(|i| (*i as i16) - null)
        .collect()
}
