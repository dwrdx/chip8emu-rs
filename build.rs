fn main() {
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/sdl2/2.30.9/lib");
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/sdl2_ttf/2.22.0/lib");
}
