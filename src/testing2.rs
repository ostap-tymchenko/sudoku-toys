pub fn main() {
    if ["foo", "bar"].iter().any(|&s| s == "foo") {
        print!("yes")
    } else {
        println!("no")
    }
}
