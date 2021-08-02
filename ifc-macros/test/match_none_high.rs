use ifc_macros::ifc_block;

fn main() {
    let a = true;
    ifc_block!{
        let mut _x = 4;
        match a {
            true => _x += 1,
            false => _x -= 1,
        }
    }
}