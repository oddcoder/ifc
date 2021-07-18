use ifc_macros::ifc_block;

fn main() {
    let x = 1;
    ifc_block!{
        #[IFC(Low)]
        let w = 2;
        let y = x + w;
        let z = 3;
    };
    assert!(z == y);
}