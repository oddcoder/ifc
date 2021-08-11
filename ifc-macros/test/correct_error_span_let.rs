use ifc_macros::ifc_block;


fn main() {
    ifc_block! {
        #[IFC(Low)]
        let w : u64 = 1;
        let x : u64 = 2;
        #[IFC(Low)]
        let z : u64 = w * x;
        println!("z = {}", z);
    };
}
