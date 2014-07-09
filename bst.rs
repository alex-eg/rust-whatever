use std::fmt::Show;

enum Bst<T> {
    Node(T, Box<Bst<T>>, Box<Bst<T>>),
    Nil
}

impl <T:PartialOrd + Show> Bst<T> {
    fn add<T>(&mut self, v: T) -> () {
        match self {
            &Nil => *self = Node(v, box Nil, box Nil),
            &Node(ref x, ref mut x_left, ref mut x_right) => {
                if v > *x { x_right.add(v) }
                else if v < *x { x_left.add(v) }
            }
        }
    }

    fn print(&self) {
        match self {
            &Nil => print!("nil"),
            &Node(ref x, ref x_left, ref x_right) =>
                { print!("Bt({}, ", x);
                  x_left.print();
                  print!(", ");
                  x_right.print();
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
