fn fib_iter(a: int, b: int, p: int, q: int, count: int) -> int {
    if count == 0 { b }
    else if count % 2 == 0 { fib_iter(a, b, q*q + p*p, q*q + 2*p*q, count / 2) }
    else { fib_iter(b * q + a * q + a * p,
                    b * p + a * q,
                    p, q, count - 1) }
}

fn fib(n: int) -> int {
    fib_iter(1, 0, 0, 1, n)
}

fn main() {
    println!("fib(5) == {}, fib(30) == {}", fib(5), fib(30));
}
