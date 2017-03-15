extern crate flare;

fn gen<T>(func_name: &str, funcs: ToIterator<Item = &str>) -> AstTree {
    let mut func = flare::fn_();
    func.named(func_name);

    let tuptypes = Vec::new();

    let n = func.arg("n", flare::ty::u32);
    let start = func.arg("start", flare::ty::f32);

    let vals = Vec::new();

    for f in funcs {
        let val = func.attach(flare::let_from(start));
        let range = flare::range(Some(0), Some(n.copied()));
        let repeat = flare::for_(range);
        repeat.attach(val.modify(val.call(f) * std::f32::consts::PI));
        func.attach(repeat);
        vals.push(val);
        tuptypes.push(&ast::ty::f32);
    }

    func.attach(flare::return_(flare::tuple_from(vals)));

    func.returns(flare::ty::tuple(&tuptypes[..]));

    func.to_ast();
}