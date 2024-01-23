$target = "wasm32-wasi"

cargo build --target $target --release

$before = [math]::ceiling((Get-ChildItem ".\target\$target\release\demo_fleet.wasm").Length / 1kb)

.\tools\binaryen\wasm-opt.exe `
    ".\target\$target\release\demo_fleet.wasm" `
    --output "output-$target.wasm" `
    --strip-dwarf `
    --asyncify --pass-arg=asyncify-imports@wasi_snapshot_preview1.sched_yield `
    --enable-bulk-memory --enable-simd `
    -O4

$after = [math]::ceiling((Get-ChildItem "output-$target.wasm").Length / 1kb)

[String]::Format("Optimised $before KB -> $after KB")