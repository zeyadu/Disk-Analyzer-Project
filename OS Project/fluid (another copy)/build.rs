fn main()
{
    use std::path::PathBuf;
    use std::env;
    println!("cargo:rerun-if-changed=src/myuifile.f1");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/myuifile.fl",out_path.join("myuifile.rs").to_str().unwrap()).expect("failed");
    
}