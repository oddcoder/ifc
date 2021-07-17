use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Unsafe)]
        let x : usize = 5;
        // This is little hack that triggers compiler errors
        // This way we can detect compiler warnings that
        // should trigger
    };
    compile_error!("Warning_Error");
}