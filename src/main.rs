use std::io::fs::PathExtensions;

fn main() {
    let path = Path::new("hoge.txt");
    println!("{}", path.exists())
}
