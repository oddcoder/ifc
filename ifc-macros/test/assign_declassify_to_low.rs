use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x;
        let a = 5;
        #[IFC(Declassify)]
        x = a;
        #[IFC(Low)]
        let y = 5;
    }
    assert_eq!(x, y);
}