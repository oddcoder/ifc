use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x = 2;
        let w = 1;
        let y = x + w;
        let z = 3;
    };
    assert!(z == y);
}