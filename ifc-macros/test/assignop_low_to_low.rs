use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x = 5;
        #[IFC(Low)]
        let mut y = 5;
        #[IFC(Low)]
        let z = 10;
        y += x;
    };
    assert_eq!(y, z);
}