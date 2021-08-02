use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let a = true;
        #[IFC(Low)]
        let mut _x = 4;
        match a {
            true => _x += 1,
            false => _x -= 1,
        }
    }
}