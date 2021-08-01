use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        let a = 5;
        if a == 5 {
            let x = 5;
            x
        } else {
            #[IFC(Low)]
            let x = 6;
            x
        };
    }
}