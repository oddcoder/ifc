use ifc_macros::ifc_block;

fn main() {
    let _x: usize;
    ifc_block!{
        let y = 5;
        x = y;
    };
}