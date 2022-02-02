fn main() {
    let ctx = sifis::Discovery::new();

    for thing in ctx.things() {
        println!("{:#?}", thing);
    }
}
