use ifc_macros::ifc_block;

fn main() {
    let mut x = 5;
    ifc_block!{
        let y = 5;
        x += y;
    };
}