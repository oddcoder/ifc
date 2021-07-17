use ifc_macros::ifc_block;

fn main() {
    let _x: usize;
    let y = 5;
    ifc_block!{
        _x = y;
    };
}