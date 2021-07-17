use ifc_macros::ifc_block;
use ifc::*;
fn main() {
    let _x: usize;
    ifc_block!{
        #[IFC(Low)]
        let y = 5;
        _x = y;
    };
}