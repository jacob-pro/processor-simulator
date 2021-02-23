COMPILE = arm-none-eabi-gcc -mthumb -mcpu=cortex-m0 -ffreestanding -fpie sim.c
DUMP = arm-none-eabi-objdump -m cortex-m0 -d

all : test

test : test.c sim.c
	${COMPILE} test.c -O0 -o test.elf
	${DUMP} test.elf > test.dump

clean :
	rm -f *.elf

ide :
	compiledb -n -o - make > compile_commands.json
