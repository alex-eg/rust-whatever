use std::io;

fn main()
{
    for line in io::stdin().lines() {
        println!("{}", line.unwrap());
    }
}
    
