use ifc_macros::ifc_block;
use ifc::*;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x = 5;
        let _y = format!("{}", x);
        let _z = vec![1,2,3,4,5];
    }
}