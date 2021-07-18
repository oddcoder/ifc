use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x = 5;
        let mut y = 5;
        let z = 10;
        y += x;
    };
    assert!(y == z);
}