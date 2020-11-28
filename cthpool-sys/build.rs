fn main() {
    cc::Build::new()
        .file("c/thpool.c")
        .compile("thpool");
}
