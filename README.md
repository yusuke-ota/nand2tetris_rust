# nand2tetris_ch7_rust
「コンピュータシステムの理論と実装」第7章のRust実装(N番煎じ)

.vmファイルから.asmコードを生成します。

## run

```shell
cargo run file\to\path\filename.vm
```

## test

### Parser

テストを実装しています。

Run

```shell
cargo test
```

### CodeWriter

生成した.vmファイルがCPUEmulator上で意図したとおりに動いているかどうかで確認してください。
