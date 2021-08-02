use ifc_macros::ifc_block;

fn main() {
    let mut _x = 4;
    ifc_block!{
        #[IFC(Low)]
        let a = true;
        match a {
            true => _x += 1,
            false => _x -= 1,
        }
    }
}