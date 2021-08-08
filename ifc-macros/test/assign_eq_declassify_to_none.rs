use ifc_macros::ifc_block;
use ifc::*;

fn main() {
    let mut x = 1;
    ifc_block!{
        let a = 5;
        #[IFC(Declassify)]
        x += a;
    }
    assert_eq!(x, 6);
}