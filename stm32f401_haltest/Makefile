target=thumbv7em-none-eabihf
elf_debug=./target/thumbv7em-none-eabihf/debug/stm32401_pactest

build:
	cargo build --target $(target)

release:
	cargo build --target $(target) --release

clean:
	rm -rf target

flash:
	openocd \
        -f interface/stlink.cfg \
        -f target/stm32f4x.cfg \
		-c "program $(elf_debug) verify reset exit"

# Seems not needed, build and release do the work.
rustc:
	cargo rustc --target $(target) -- \
      -C linker=arm-none-eabi-ld

# binary is needed to flash using STM32CubeProgrammer
objcopy:
	arm-none-eabi-objcopy -v -O binary $(elf_debug) foo.bin

.phony: build clean flash rustc objcopy

