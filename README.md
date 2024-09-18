# Kerosine

![Animation of someone (Maurice, from The IT Crowd) using a computer while their desk is on fire](kerosine.gif)

🦀 Hello, and welcome to Kerosine!

This project is my first time trying to build some neural networks in Rust, using Hugging Face's [Candle library](https://github.com/huggingface/candle). I can't quite claim I know what I'm doing, so watch out for fire and let me know if anything is burning.

## Using It

As of now, Candle doesn't support the latest version of Cuda, 12.6, so we need to clone Candle and modify its Cargo.toml. This is true for both Linux and macOS, even though Macs don't use Nvidia chips, because Cargo wants the same path for modules that have the same name. So it goes.

Making the modification is a sed one-liner that tells Candle to use the latest version of [cudarc](https://crates.io/crates/cudarc).

```shell
$ git clone https://github.com/jmsdnns/kerosine
$ git clone https://github.com/huggingface/candle
$ sed -i 's/cudarc = { version = "0.12.0"/cudarc = { version = "0.12.1"/' candle/Cargo.toml
```

