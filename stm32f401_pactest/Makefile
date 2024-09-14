debug_target=./target/thumbv7em-none-eabihf/debug/stm32401_pactest
build:
	cargo build --target thumbv7em-none-eabihf
release:
	cargo build --target thumbv7em-none-eabihf --release
clean:
	rm -rf target
flash:
	openocd -f interface/stlink.cfg -f $(debug_target) -c "program your_file.elf verify reset exit"
.phony: build clean flash
