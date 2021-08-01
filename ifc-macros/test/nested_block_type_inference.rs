use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        let x = 5;
        #[IFC(Low)]
        let a = {
            #[IFC(Low)]
            let x = 4;
            x + 1
        };
        let y = a;
    }
    assert!(x == y);
}