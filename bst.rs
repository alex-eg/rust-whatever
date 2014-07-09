use std::fmt::Show;
use std::option::Option;

enum Bst<T> {
    Node(T, Box<Bst<T>>, Box<Bst<T>>),
    Nil
}

impl <T:Ord + Show> Bst<T> {
    fn search(&self, v: T) -> Option<T> {
        match self {
            &Nil => None,
            &Node(ref x, ref left, ref right) => {
                if v == *x { Some(v) }
                else if v < *x { left.search(v) }
                else { right.search(v) }
            }
        }
    }

    fn add(&mut self, v: T) -> () {
        match self {
            &Nil => *self = Node(v, box Nil, box Nil),
            &Node(ref x, ref mut left, ref mut right) => {
                if v > *x { right.add(v) }
                else if v < *x { left.add(v) }
            }
        }
    }

    fn print(&self) {
        match self {
            &Nil => print!("nil"),
            &Node(ref x, ref left, ref right) => {
                print!("Bt({}, ", x);
                left.print();
                print!(", ");
                right.print();
                print!(")");
            }
        }
    }
}

fn main()
{
    let mut a: Bst<int> = Nil;
    a.add(3);
    a.add(1);
    a.add(2);
    a.add(4);
    a.add(0);
    a.print();
    println!("");
    println!("searching 3: {}", a.search(3));
    println!("searching 0: {}", a.search(0));
    println!("searching 7: {}", a.search(7));
}
