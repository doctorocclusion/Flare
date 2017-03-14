fn create_fib(name: &str, count: u32, ty: flare::Ty, comb: Fn(Box<flare::Expressible>) -> flare::Expressible) {
    let mut func = flare::fn_();
    func.named(name);

    let n = func.arg(flare::ty::u32).named("n");

    let args: Vec<_> = (0..count).map(|i| func.arg(ty).named(&format!("initial{}", i + 1))).collect();
    let list = func.attach(flare::let_from(flare::vec(&args[..])));

    let repeat = flare::for_(flare::range(Some(0), Some(n)));
    let block = flare::block();

    let vals = block.attach(
        flare::let_from(list[repeat.val..].borrowed())
    );

    block.last(comb(Box::new(vals)));

    out = repeat.attach(flare::let_from(block));
    repeat.attach(list.call("push").arg(out));

    func.attach(repeat);
    func.last(list);

    func.to_ast()
}

create_fib("fib", 2, flare::ty::f32, |e| -> {
    e.index(0) + e.index(1)
});

/*
fn fib(n: u32, initial1: f32, initial2: f32) {
    let list = vec![initial1, initial2];
    for 0..n {
        let out = {
            let vals = &list[n..];
            val[0] + val[1]
        };
        list.push(out);
    }
    list
}








*/