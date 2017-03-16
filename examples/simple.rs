extern crate flare;

fn main() {
    let fun = flare::function();
    let a = fun.arg(flare::ty::i32);
    let v = fun.attach(flare::let_from(a + 10));
    fun.last(flare::tuple().with(v * 2).with(v));
}