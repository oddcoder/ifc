use ifc_macros::ifc_block;

fn main() {
    let x = 5;
    ifc_block!{
        #[IFC(Low)]
        let _y;
        _y = x;
    };
}