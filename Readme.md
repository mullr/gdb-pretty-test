Run this with:

    cargo build
    rust-gdb target/debug/gdb-pretty-test -ex 'b test' -ex 'r' -ex 'n' -ex 'n' -ex 'p bare_vec' -ex 'p newtype_vec'
