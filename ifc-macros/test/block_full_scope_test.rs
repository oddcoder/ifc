use ifc_macros::ifc_block;
fn main() {
    ifc_block!{
        let x = 5;
        {
            #[IFC[Low]]
            let x = 6;
            #[IFC[Low]]
            let _y = x;
        }
        let _y = x;
    }
}