COMPILE = arm-linux-gnueabi-gcc -mthumb -mcpu=cortex-m0 -ffreestanding -fpie -nostdlib --entry=start libc.c
DUMP = arm-linux-gnueabi-objdump -m cortex-m0 -d

all : basic

basic : basic.c libc.c
	${COMPILE} basic.c -o basic.elf
	${DUMP} basic.elf > basic.dump

clean :
	rm -f *.elf

ide :
	compiledb -n -o - make > compile_commands.json
