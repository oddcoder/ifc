use ifc_macros::ifc_block;

fn main() {
    let a = 5;
    let b = 6;
    ifc_block!{
        #[IFC(Low)]
        let c = if a == 5 {
            let _x = 5;
            #[IFC(Low)]
            let _y = 6;
            11
        } else {
            let _x = 5;
            #[IFC(Low)]
            let _y = 6;
            12
        };

        #[IFC(Low)]
        let d = if b == 7 {
            let _x = 5;
            #[IFC(Low)]
            let _y = 6;
            12
        } else {
            let _x = 5;
            #[IFC(Low)]
            let _y = 6;
            11
        };
    }
    assert_eq!(c, d);
}