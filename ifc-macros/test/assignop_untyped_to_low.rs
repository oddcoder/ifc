use ifc_macros::ifc_block;

fn main() {
    let x = 5;
    ifc_block!{
        #[IFC(Low)]
        let mut y = 5;
        #[IFC(Low)]
        let z  = 10;
        y += x;
    };
    assert_eq!(y, z);
}