pub fn generate_cs_bindings() {
    csbindgen::Builder::default()
        .csharp_dll_name("zenohc")
        .csharp_namespace("Zenoh")
        .csharp_class_name("ZenohFFI")
        .input_extern_file("src/lib.rs")
        .generate_csharp_file("csoutput/zenoh.cs")
        .unwrap();
}