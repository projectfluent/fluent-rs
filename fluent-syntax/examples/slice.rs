fn main() {
    let s = include_str!("../benches/menubar.ftl");
    println!("{:#?}", &s[6796..6799]);
}
