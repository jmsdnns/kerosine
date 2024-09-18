# Kerosine

![Animation of someone (Maurice, from The IT Crowd) using a computer while their desk is on fire](kerosine.gif)

🦀 Hello, and welcome to Kerosine!

This project is my first time trying to build some neural networks in Rust. To keep things simple, the plan is to port over some of the NNs in my [Pallets](https://github.com/jmsdnns/pallets) project. That project depends on the [cpunks-10k dataset](https://github.com/tnn1t1s/cpunks-10k) collected by [@tnn1t1s](https://github.com/tnn1t1s), so this one does too. Kerosine is built with a minimal ML framework from 🤗Hugging Face, called [Candle](https://github.com/huggingface/candle).

I can't claim I know what I'm doing yet, so watch out for fire and let me know if anyone or anything is burning.

## Using It

Kerosine tries to avoid making assumptions, but one assumption it has is that kerosine and cpunks-10k are in the same directory.

```shell
$ git clone https://github.com/jmsdnns/kerosine
$ git clone https://github.com/tnn1t1s/cpunks-10k
```

> ![IMPORTANT]
> Candle is behind in its support of CUDA so additional steps are required. 

As of now, Candle doesn't support the latest version of Cuda, 12.6. We can address this by getting Candle from Github and modifying which version of [cudarc](https://crates.io/crates/cudarc) it uses. Unfortunately, we must do this for both Linux and macOS, even though Macs don't use Nvidia chips. Fortunately, this is easy with a sed one-liner.

```shell
$ git clone https://github.com/huggingface/candle
$ sed -i 's/cudarc = { version = "0.12.0"/cudarc = { version = "0.12.1"/' candle/Cargo.toml
```

At this point, Kerosine should build correctly.
