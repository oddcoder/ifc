use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        let a = 5;
        #[IFC(Low, Declassify)]
        let x = a;
        #[IFC(Low)]
        let y = 5;
    }
    assert_eq!(x, y);
}