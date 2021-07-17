use ifc_macros::ifc_block;
use ifc::*;

fn add1(x: usize, y: usize) -> usize {
    x + y
}
fn add2(x: &usize, y: &usize) -> usize {
    *x + *y
}
fn inc(x: &mut usize) {
    *x+= 1
}
fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x : usize = 5;
        #[IFC(Low)]
        let y : usize = 5;
        let _a = add1(x, y);
        let _b = add2(&x, &y);
        inc(&mut x);
    };
}