
# Put this in your ~/.bashrc file, and update the path to point to your built version

function mktmp() {
    eval `$HOME/devel/rust_utils/mktmp/target/debug/mktmp $1`
}
