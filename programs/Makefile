COMPILE = arm-none-eabi-gcc -mthumb -mcpu=cortex-m0 -ffreestanding --entry=main -fpie sim.c
DUMP = arm-none-eabi-objdump -m cortex-m0 -d

all : basic

basic : basic.c sim.c
	${COMPILE} basic.c -o basic.elf
	${DUMP} basic.elf > basic.dump

clean :
	rm -f *.elf

ide :
	compiledb -n -o - make > compile_commands.json
