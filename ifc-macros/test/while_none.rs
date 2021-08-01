use ifc_macros::ifc_block;

fn main() {
    let mut a = 5;
    let mut _x = 0;
    ifc_block!{
        while a > 5 {
            a -= 1;
            _x += 1;
        }
    }
}