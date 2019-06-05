      lis $1
      .word 0xFFFF0004
      lis $2
      .word 0xFFFF000C
      lis $3
      .word 0x1B
loop: lw $4, 0($1)
      sw $4, 0($2)
      bne $3, $4, loop
      jr $31