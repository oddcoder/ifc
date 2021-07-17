use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low)]
        let x = 5;
    };

    let x_str = format!("{}", x);
    assert_eq!(x_str, "5");
}