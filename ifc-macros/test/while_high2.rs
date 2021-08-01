use ifc_macros::ifc_block;

fn main() {
    let mut _x = 0;    
    ifc_block!{
        let mut a = 5;
        while a > 5 {
            a -= 1;
            _x += 1;
        }
    }
}