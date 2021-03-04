COMPILE = arm-none-eabi-gcc -mthumb -mcpu=cortex-m0 -ffreestanding sim.c
DUMP = arm-none-eabi-objdump -m cortex-m0 -d

all : test fibonacci factorial

test : test.c sim.c
	${COMPILE} test.c -O0 -o test.elf
	${DUMP} test.elf > test.dump

fibonacci : fibonacci.c sim.c
	${COMPILE} fibonacci.c -o fibonacci.elf
	${DUMP} fibonacci.elf > fibonacci.dump

factorial : factorial.c sim.c
	${COMPILE} factorial.c -o factorial.elf
	${DUMP} factorial.elf > factorial.dump

clean :
	rm -f *.elf

ide :
	compiledb -n -o - make > compile_commands.json
