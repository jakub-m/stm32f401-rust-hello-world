Tried [stm32f3-rust-hello-world][ref1], did not manage to build it.

Based on [STM32F4xx VSCode Project Setup][ref2].

Board manual from ST [here (pdf)](https://www.st.com/en/evaluation-tools/nucleo-f401re.html#documentation).

MCU manual from ST [here (pdf)](https://www.st.com/en/microcontrollers-microprocessors/stm32f401re.html#documentation).


Install [Rust][ref_rust].

Install OpenOCD:

```
brew install \
  openocd \
  arm-none-eabi-binutils \
  arm-none-eabi-gdb \
```

```
cargo install svd2rust
```

Useful:

```
# Provides readelf
brew install binutils
```

HAL example from [this page](https://github.com/stm32-rs/stm32f4xx-hal?tab=readme-ov-file)

Install SVD from [STM](https://www.st.com/content/st_com/en/search.html#q=svd-t=resources-page=1).

Followed [this post on dev.to](https://dev.to/theembeddedrustacean/stm32f4-embedded-rust-at-the-pac-svd2rust-457d)

Blinking diode from [this post on
dev.to](https://dev.to/theembeddedrustacean/stm32f4-embedded-rust-at-the-pac-gpio-control-20h4)


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
