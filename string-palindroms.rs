use std::io;

fn reverse(s: &~str) -> ~str {
    s.chars_rev().collect()
}

fn get_palindroms(s: ~str) -> ~[~str] {
    let mut words = ~[];
    for q in s.words() {
        words.push(q.to_owned());
    }
    let mut result = ~[];
    let mut p = words.iter().filter(|&x| *x == reverse(x));
    for q in p {
        result.push(q.to_owned());
    }
    result   
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
