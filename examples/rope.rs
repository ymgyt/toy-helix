fn main() {
    let mut f = std::fs::File::open("./src/application.rs").unwrap();

    let mut r = ropey::Rope::from_reader(&mut f).unwrap();

    let c = r.line_to_char(1);
    println!("line=1, char={c}");
}
