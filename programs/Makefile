COMMAND = arm-linux-gnueabi-gcc -march=armv6-m -mfloat-abi=hard -ffreestanding -nostdlib --entry=start

all : basic

basic : basic.c libc.c
	${COMMAND} basic.c libc.c -o basic.elf

clean :
	rm -f *.elf

ide :
	compiledb -n -o - make > compile_commands.json
