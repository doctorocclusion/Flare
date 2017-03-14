fn main() {
	let v = flare::literal(16.);
	let s = v.call("log").arg(2.);
	s.modify(s / 4.);

	s.as_expr().to_ast();

	let expr = flare::expr(s);
}