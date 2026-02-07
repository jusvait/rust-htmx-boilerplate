fn main() {
    println!("cargo:rerun-if-changed=templates/");

    let dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = format!("{dir}/.style.css");
    let output = format!("{dir}/static/style.css");

    let result = std::process::Command::new("npx")
        .args(["tailwindcss", "--minify", "-i", &input, "-o", &output])
        .output()
        .expect("Could not generate css");

    if !result.status.success() {
        let error = String::from_utf8_lossy(&result.stderr);
        println!("cargo::error=Error while building CSS");
        println!("cargo::error=Output: {error}");
    }
}
