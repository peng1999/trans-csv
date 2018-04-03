mod sundown;

fn main() {
    let s = sundown::render("23");
    println!("{}", s.as_ref());
}
