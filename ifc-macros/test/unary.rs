use ifc_macros::ifc_block;
fn main() {
    ifc_block!{
        #[IFC(Low)]
        let w = 5;
        #[IFC(Low)]
        let x = -w;
        let y = -x;
        // Would be cool to have type inference 
        let _z : i32;
        _z = -y;
    };
}