# Usage

安裝完RUST後我們可以開始寫我們第一個RUST得程式了，首先我們先下`cargo init`這將會替我們創建一些基本的檔案。

```bash=
$ cargo init
```

目錄應該是長下面這樣，`Cargo.toml`裡面寫的是一些設定檔案，版本訊息等等，`main.rs`則是我們主要寫程式的地方。
```
.
|-- Cargo.toml
`-- src
    `-- main.rs
```

然後打開`main.rs`編寫我們第一個RUST程式。
```rust=
fn main() {
    println!("Hello, world!");
}
```

接著執行程式

```bash=
$ cargo run
```

or

```bash=
cargo run --bin filename
```