use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let mut x = 1;
        let a = 5;
        #[IFC(Declassify)]
        x += a;
        #[IFC(Low)]
        let y = 6;
    }
    assert_eq!(x, y);
}