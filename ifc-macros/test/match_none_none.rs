use ifc_macros::ifc_block;

fn main() {
    let a = true;
    let mut _x = 4;
    ifc_block!{
        match a {
            true => _x += 1,
            false => _x -= 1,
        }
    }
}