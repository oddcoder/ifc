use ifc_macros::ifc_block;

fn main() {
    let x = 5;
    ifc_block!{
        let mut y = 5;
        let z = 10;
        y += x;
    };
    assert!(y == z);
}