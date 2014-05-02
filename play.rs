fn reverse(s: &~str) -> ~str {
    s.chars_rev().collect()
}

fn main()
{
    let k = ~[1,2,3,4];
    let w = "just a string";
    let e = ~"owned string";
    println!("{} -- {}", k, w);
    println!("{}", reverse(&e));
}
