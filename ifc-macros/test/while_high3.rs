use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        let mut a = 5;
        let mut _x = 0;    
        while a > 5 {
            a -= 1;
            _x += 1;
        }
    }
}