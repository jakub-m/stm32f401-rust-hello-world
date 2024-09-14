Based on [stm32f3-rust-hello-world][ref1] (tried, didn't manage to build).

Based on [STM32F4xx VSCode Project Setup][ref2]


Install [Rust][ref_rust].

Install OpenOCD:

```
brew install openocd
```

Install [xargo][ref_xargo]


[ref1]:https://github.com/phreaknik/stm32f3-rust-hello-world
[ref2]:https://apollolabs.notion.site/STM32F4xx-VSCode-Project-Setup-3cdea2ce79f34a08a1a7e3f987e992a7
[ref_rust]:https://www.rust-lang.org/tools/install
[ref_xargo]:https://github.com/japaric/xargo

To switch between [stable and nightly][ref_stable]

```
rustup default nightly
rustup default stable
```

[ref_stable]:https://users.rust-lang.org/t/how-to-switch-between-rust-stable-version-and-nightly-verison-in-vscode/61429

```
rustup component add rust-src --toolchain nightly
```


```
touch Cargo.lock
xargo generate-lockfile
```

xargo versions

0.3.26 - error
0.3.16 - `rust-src` error
0.3.10 - `rust-src` error
0.3.8 - `error: `rust-src` component not found. Run `rustup component add rust-src`
0.2.3  - good


% RUST_LOG=debug RUST_BACKTRACE=1 xargo build
