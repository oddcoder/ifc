use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        let a = 5;
        if a == 5 {
            #[IFC(Low)]
            let x = 5;
            x
        } else {
            let x = 6;
            x
        };
    }
}