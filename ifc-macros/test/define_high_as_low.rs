use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x : usize = 5;
        let _y: usize  = x;
    };
}