enum Bst {
    Node(i32, Box<Bst>, Box<Bst>),
    Nil
}

impl Bst {
    fn add(& mut self, v: i32) {
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
                { print!("Bt({},", x);
                  x_left.print();
                  print!(",");
                  x_right.print();
                  print!(")");
                }
        }
    }
}

fn main()
{
    let mut a = Nil;
    a.add(3);
    a.add(1);
    a.add(2);
    a.add(4);
    a.print();
}
