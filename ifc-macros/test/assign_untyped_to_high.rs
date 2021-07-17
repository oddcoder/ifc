use ifc_macros::ifc_block;

fn main() {
    let x = 5;
    ifc_block!{
        let mut _y = 6;
        _y = x;
    };
}