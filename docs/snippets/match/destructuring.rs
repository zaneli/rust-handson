let x = (1, 5);
let label = match x {
    (0, 0) => "ゼロ",
    (1, 0) | (0, 1) => "単位",
    (1, _) => "x は 1",
    (_, 1) => "y は 1",
    _ => "その他"
};
println!("{}", label);