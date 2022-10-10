rm -rf package_names.txt
rm dump.json
rm -f
cargo build --release
$bin=target/debug/release
$bin write-dump
$bin extract-package-names
$bin dump-versions-and-ranges
$bin test-versions-and-ranges