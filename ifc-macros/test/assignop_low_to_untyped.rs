use ifc_macros::ifc_block;
use ifc::*;
fn main() {
    let mut x: usize = 5;
    ifc_block!{
        #[IFC(Low)]
        let y = 5;
        x += y;
    };
    assert_eq!(x, 10);
}