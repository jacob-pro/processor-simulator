arm-linux-gnueabi-gcc -march=armv6-m -mfloat-abi=hard -ffreestanding -nostdlib basic.c --entry=start -o basic.elf && arm-linux-gnueabi-objdump -d basic.elf > output.objdump

