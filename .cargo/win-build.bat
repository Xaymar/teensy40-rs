SET "ELF=./target/thumbv7em-none-eabihf/release/teensy_audio_led"
SET "HEX=./target/thumbv7em-none-eabihf/release/teensy_audio_led.hex"

cargo build --release

arm-none-eabi-objcopy -O ihex %ELF% %HEX%
