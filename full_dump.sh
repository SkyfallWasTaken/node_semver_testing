rm out > /dev/null
set -o pipefail
mkdir out
cargo build --release
$bin=target/debug/release
$bin write-dump
$bin extract-package-names
$bin dump-versions-and-ranges
$bin test-versions-and-ranges