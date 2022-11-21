use std::collections::HashSet;

pub fn main () {
    let digits: i32 = 123456789.parse();
    let rows = "ABCDEFGHI";
    let cols = digits;
    
   fn cross (a:&str, b:&str) -> Vec<String> {
        let mut squares = vec![];
        for a_char in a.chars() {
            for b_char in b.chars() {
                squares.push(format!("{}{}", a_char, b_char));
            }
        }
        squares
    }

    let squares = cross(&rows, &cols);
    dbg!(digits, &squares); 
    assert_eq!(81, (squares).len())
    //let unitlist: HashSet::new(); 
}
