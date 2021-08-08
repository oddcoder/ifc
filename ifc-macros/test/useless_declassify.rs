use ifc_macros::ifc_block;

fn main() {
    ifc_block!{
        #[IFC(Low, Declassify)]
        let _a = 5;
        #[IFC(Low, Declassify)]
        let _x = 4;
    }
    compile_error!("");
}