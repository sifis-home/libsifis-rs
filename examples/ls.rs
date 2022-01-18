fn main() {
    let ctx = sifis::Context::new();

    for thing in ctx.things() {
        println!("{:#?}", thing);
    }
}
