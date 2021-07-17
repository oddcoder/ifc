use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x = 5;
        let mut _y = 6;
        _y = x;
    };
}