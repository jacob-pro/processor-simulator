# Experiments

## One

Hypothesis: Using a 3 stage pipeline will reduce the total cycles by up to a maximum factor of 3. The less taken branches the closer it will be to the maximum reduction.

```
cargo run -- programs/bitcount.elf -s scalar
cargo run -- programs/bitcount.elf -s pipelined

cargo run -- programs/bitcount_unrolled.elf -s scalar
cargo run -- programs/bitcount_unrolled.elf -s pipelined
```

| Program           | Instructions | Branches Taken | Non-Pipelined Cycles | Pipelined Cycles | Factor |
|-------------------|--------------|----------------|----------------------|------------------|--------|
| Bitcount          | 1611         | 100            | 5467                 | 2447             | 2.234  |
| Bitcount Unrolled | 1067         | 56             | 3649                 | 1629             | 2.240  |
