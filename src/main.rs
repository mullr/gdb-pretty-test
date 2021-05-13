
struct TrivialNewtype(Vec<u8>);

fn test() {
    let bare_vec = vec![1u8, 2, 3];
    let newtype_vec = TrivialNewtype(vec![1u8, 2, 3]);
    let placeholder = 12;
}

fn main() {
    test()
}
