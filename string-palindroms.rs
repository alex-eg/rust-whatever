use std::io;

fn reverse(s: ~str) -> ~str {
    s.chars_rev().collect()
}

fn get_palindroms(s: ~str) -> ~[~str] {
    let words:~[~str] = s.split_terminator(' ').collect();
    let p : ~[~str] = words.iter().filter(|x| reverse(*x) == x).collect();
    p
}   

fn main() {
    for line in io::stdin().lines() {
        let s = line.unwrap();
        let p_list = get_palindroms(s);
        if p_list.len() == 0 {
            println!("No palindroms found!");
        } else {
            println!("Found palindroms:");
            for p in p_list.move_iter() {
                println!("{}", p);
            }
        }
    }
}
