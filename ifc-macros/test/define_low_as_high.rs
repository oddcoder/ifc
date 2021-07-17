use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        let x : usize = 5;
        #[IFC(Low)]
        let _y: usize  = x;
    };
}