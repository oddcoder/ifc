use ifc_macros::ifc_block;
fn main() {
    ifc_block!{
        let x = 5;
        {
            // this works best as a failing test
            // since we need to make sure that x
            // is not typed as None and instead read from
            // higher scope which is typed as High
            #[IFC[Low]]
            let _y = x;
        }
    }
}