use ifc_macros::ifc_block;

fn main() {
    let w = 1;
    let x = 2;
    ifc_block!{
        let y = x + w;
        let z = 3;
    };
    assert!(z == y);
}