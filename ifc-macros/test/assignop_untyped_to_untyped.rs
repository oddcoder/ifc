use ifc_macros::ifc_block;

fn main() {
    let mut x: usize = 5;
    let y = 5;
    let z = 10;
    ifc_block!{
        x += y;
    };
    assert_eq!(x, z);
}