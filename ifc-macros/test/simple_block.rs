use ifc_macros::ifc_block;
fn main() {
    ifc_block!{
        let _x = 5;
        {
            let _y = 6;
        }
    }
}