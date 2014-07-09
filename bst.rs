use std::fmt::Show;
use std::option::Option;

enum Bst<T> {
    Node(T, Box<Bst<T>>, Box<Bst<T>>),
    Nil
}

impl <T:Ord + Show> Bst<T> {
    fn search<T>(&self, v: T) -> Option<T> {
        match self {
            &Nil => None,
            &Node(x, ref left, ref right) => {
                if v == x { Some(x) }
                else if v < x { left.search(x) }
                else if v > x { right.search(x) }
            }
        }
    }

    fn add<T>(&mut self, v: T) -> () {
        match self {
            &Nil => *self = Node(v, box Nil, box Nil),
            &Node(x, ref mut left, ref mut right) => {
                if v > x { right.add(v) }
                else if v < x { left.add(v) }
            }
        }
    }

    fn print(&self) {
        match self {
            &Nil => print!("nil"),
            &Node(x, ref left, ref right) =>
                { print!("Bt({}, ", x);
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
    a.add(3i);
    a.add(1);
    a.add(2);
    a.add(4);
    a.print();
    println!("");
}
