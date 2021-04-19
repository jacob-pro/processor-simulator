# Experiments

## One

Hypothesis: Using a 3 stage pipeline will reduce the total cycles by up to a maximum factor of 3. The less taken branches the closer it will be to the maximum reduction.

```
cargo run -- programs/bitcount_o3.elf -s scalar
cargo run -- programs/bitcount_o3.elf -s pipelined

cargo run -- programs/bitcount_unrolled_o3.elf -s scalar
cargo run -- programs/bitcount_unrolled_o3.elf -s pipelined
```

| Program           | Instructions | Branches Taken | Non-Pipelined Cycles | Pipelined Cycles | Factor |
|-------------------|--------------|----------------|----------------------|------------------|--------|
| Bitcount O3          | 630         | 62            | 1896                 | 762             | 2.488  |
| Bitcount Unrolled O3 | 433         | 16             | 1306                 | 474             | 2.755  |

## Two

Hypothesis: Adding an execution unit / reservation station will increase the instructions per cycle rate, each time by a decreasing amount, until reaching zero.

```
cargo run -- programs/test2.elf -s outoforder -u 1
cargo run -- programs/test2.elf -s outoforder -u 2
cargo run -- programs/test2.elf -s outoforder -u 3
cargo run -- programs/test2.elf -s outoforder -u 4
cargo run -- programs/test2.elf -s outoforder -u 5
cargo run -- programs/test2.elf -s outoforder -u 6
```

| Stations / Units     | 1     | 2     | 3     | 4     | 5     | 6     | 7     |
|----------------------|-------|-------|-------|-------|-------|-------|-------|
| Cycles               | 34294 | 30847 | 29924 | 29793 | 29780 | 29772 | 29772 |
| Change               |       | -3447 | -923  | -131  | -13   | -8    | 0     |
| Instructions / Cycle | 0.581 | 0.646 | 0.651 | 0.654 | 0.654 | 0.654 | 0.654 |


## Three

Hypothesis: Using loop unrolling to increase instruction level parallelism will lead to a greater improvement in cycle rate when adding additional execution units.

```
cargo run -- programs/bitcount_o0.elf -s outoforder -u 1
cargo run -- programs/bitcount_unrolled_o0.elf -s outoforder -u 1

cargo run -- programs/bitcount_o0.elf -s outoforder -u 4
cargo run -- programs/bitcount_unrolled_o0.elf -s outoforder -u 4
```

| Program              | Instructions | 1 Unit | 4 Units | Change |
|----------------------|--------------|--------|---------|--------|
| Bitcount O0          | 1611         | 0.647  | 0.840   | +29.8% |
| Bitcount Unrolled O0 | 1067         | 0.637  | 0.836   | +31.3% |
