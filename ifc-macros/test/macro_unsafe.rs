use ifc_macros::ifc_block;
use ifc::*;

fn main() {
    ifc_block!{
        let x = 5;
        #[IFC(Unsafe)]
        let _y = format!("{}", x);
    }
}