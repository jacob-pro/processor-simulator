COMPILE = arm-none-eabi-gcc -mthumb -mcpu=cortex-m0 -ffreestanding sim.c
DUMP = arm-none-eabi-objdump -m cortex-m0 -d

all : test1 test2 fibonacci factorial bitcount_o0 bitcount_o3 bitcount_unrolled_o0 bitcount_unrolled_o3

test1 : test1.c sim.c
	${COMPILE} test1.c  -nostdlib -DNOSTDLIB --entry=start -o test1.elf
	${DUMP} test1.elf > test1.dump

test2 : test2.c sim.c
	${COMPILE} test2.c -o test2.elf
	${DUMP} test2.elf > test2.dump

fibonacci : fibonacci.c sim.c
	${COMPILE} fibonacci.c -o fibonacci.elf
	${DUMP} fibonacci.elf > fibonacci.dump

factorial : factorial.c sim.c
	${COMPILE} factorial.c -o factorial.elf
	${DUMP} factorial.elf > factorial.dump

bitcount_o0 : bitcount.c sim.c
	${COMPILE} bitcount.c -nostdlib -DNOSTDLIB --entry=start -O0 -o bitcount_o0.elf
	${DUMP} bitcount_o0.elf > bitcount_o0.dump

bitcount_unrolled_o0 : bitcount.c sim.c
	${COMPILE} bitcount.c -nostdlib -DNOSTDLIB -DUNROLLED --entry=start -O0 -o bitcount_unrolled_o0.elf
	${DUMP} bitcount_unrolled_o0.elf > bitcount_unrolled_o0.dump

bitcount_o3 : bitcount.c sim.c
	${COMPILE} bitcount.c -nostdlib -DNOSTDLIB --entry=start -O3 -o bitcount_o3.elf
	${DUMP} bitcount_o3.elf > bitcount_o3.dump

bitcount_unrolled_o3 : bitcount.c sim.c
	${COMPILE} bitcount.c -nostdlib -DNOSTDLIB -DUNROLLED --entry=start -O3 -o bitcount_unrolled_o3.elf
	${DUMP} bitcount_unrolled_o3.elf > bitcount_unrolled_o3.dump

clean :
	rm -f *.elf
	rm -f *.dump

ide :
	compiledb -n -o - make > compile_commands.json
