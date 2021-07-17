use ifc_macros::ifc_block;

fn inc(x: &mut usize) {
    *x += 1;
}
fn main() {
    ifc_block!{
        let x : usize = 5;
        inc(x);
    };
}