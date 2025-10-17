## Rust

https://doc.rust-lang.org/rust-by-example/index.html

###  Ownership System

- Cada valor tem um único dono
- Quando o dono sai de escopo → memória liberada automaticamente
- Sem GC

```rust
fn main() {
    let s = String::from("Rust");
    println!("{}", s);
    // variável s não existe mais depois daqui
}
```