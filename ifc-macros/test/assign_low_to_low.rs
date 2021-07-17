use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x = 5;
        #[IFC(Low)]
        let _y;
        _y = x;
    };
}