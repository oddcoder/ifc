use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        let _x : usize = 5;
        #[IFC(High)]
        let _y = 6;
        #[IFC(Low)]
        let _z = 7;
        let mut m : Vec<u8> = Vec::new();
    };
    // This is little hack that triggers compiler errors
    // This way we can detect compiler warnings that
    // should trigger
    compile_error!("Warning_Error");
}