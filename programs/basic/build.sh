arm-linux-gnueabi-gcc -ffreestanding -nostdlib -mthumb -march=armv6 basic.c -lgcc --entry=start -o basic.elf && arm-linux-gnueabi-objdump -D basic.elf > output.objdump

