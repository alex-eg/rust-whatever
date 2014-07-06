fn main()
{
    let mut lines = [];
    let reader = io::sidin();
    println!("Input number: ");
    while !reader.eof() {
        allLines.push((reader as io:ReaderUtil).read_line());
    }
    
    for allLines.each |line| {
        println!("{}", line);
    }
}
    
