# nand2tetris_ch7_ch8_rust
「コンピュータシステムの理論と実装」第7章, 第8章のRust実装(N番煎じ)

.vmファイルから.asmコードを生成します。

## run

```shell
cargo run file\to\path\filename.vm
```
or
```shell
cargo run folder\to\path\target_folder
```

## test

### Parser

テストを実装しています。

Run

```shell
cargo test --verbose --package parser --lib parser::tests
```

### CodeWriter

生成した.vmファイルがCPUEmulator上で意図したとおりに動いているかどうかで確認してください。
