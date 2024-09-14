target=thumbv7em-none-eabihf
elf_debug=./target/thumbv7em-none-eabihf/debug/stm32401_pactest

build:
	cargo build --target $(target)

release:
	cargo build --target $(target) --release

clean:
	rm -rf target

flash:
	openocd -f interface/stlink.cfg -f $(elf_debug) -c "program your_file.elf verify reset exit"

rustc:
	cargo rustc --target $(target) -- \
      -C linker=arm-none-eabi-ld

objcopy:
	arm-none-eabi-objcopy -v -O binary $(elf_debug) foo.bin

.phony: build clean flash rustc objcopy

