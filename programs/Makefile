COMPILE = arm-linux-gnueabi-gcc -march=armv6-m -mfloat-abi=hard -ffreestanding -nostdlib --entry=start
DUMP = arm-linux-gnueabi-objdump -d

all : basic

basic : basic.c libc.c
	${COMPILE} basic.c libc.c -o basic.elf
	${DUMP} basic.elf > basic.dump

clean :
	rm -f *.elf

ide :
	compiledb -n -o - make > compile_commands.json
