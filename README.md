[TypeScriptやGoやRustでWebAssemblyウォークスルー - SMARTCAMP Engineer Blog](https://tech.smartcamp.co.jp/entry/wasm-walkthrough)
にあった Rust で WebAssembly のサンプル。 

これは簡単ですごい。大したもんだ。

Goの [syscall/js](https://pkg.go.dev/syscall/js) と wasm_exec.js
とか
[assemblyscript loader](https://github.com/AssemblyScript/assemblyscript/tree/main/lib/loader)
みたいな
汎用でないコードを使うのが
かっこいい。

欠点といえば
`cargo install wasm-pack`
がWindowsだと死ぬことかな。
(Is `gcc.exe` installed?って言われるので、
あとでもう一度試す)

このプロジェクトの生成手順(抜書)
```bash
cargo install wasm-pack
cargo new --lib sieve
# Cargo.toml編集
# src/lib.rs編集
# index.html編集
wasm-pack build --target web
```

Windowsでは
[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
にビルド済みのものがあった。 `rustup`があればOK (chocoとかで入れたrustではだめ)。
