use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        let a = 5;
        let b = 5;
        let c = 10;
        let d = (a + b) * c;
        let e = 100;
    }
    assert!(d==e);
}