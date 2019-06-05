```
I 0001 00ss ssst tttt iiii iiii iiii iiii beq   s t i
I 0001 01ss ssst tttt iiii iiii iiii iiii bne   s t i
I 0010 00ss ssst tttt iiii iiii iiii iiii addi  t s i
I 1000 11ss ssst tttt iiii iiii iiii iiii lw    t i(s)
I 1010 11ss ssst tttt iiii iiii iiii iiii sw    t i(s)
J 0000 10ii iiii iiii iiii iiii iiii iiii j     i
J 0000 11ii iiii iiii iiii iiii iiii iiii jal   i
R 0000 0000 0000 0000 dddd d000 0001 0000 mfhi  d
R 0000 0000 0000 0000 dddd d000 0001 0010 mflo  d
R 0000 0000 0000 0000 dddd d000 0001 0100 lis   d
R 0000 00ss sss0 0000 0000 0000 0000 1000 jr    s
R 0000 00ss sss0 0000 0000 0000 0000 1001 jalr  s
R 0000 00ss ssst tttt 0000 0000 0001 1000 mult  s t
R 0000 00ss ssst tttt 0000 0000 0001 1001 multu s t
R 0000 00ss ssst tttt 0000 0000 0001 1010 div   s t
R 0000 00ss ssst tttt 0000 0000 0001 1011 divu  s t
R 0000 00ss ssst tttt dddd d000 0010 0000 add   d s t
R 0000 00ss ssst tttt dddd d000 0010 0010 sub   d s t
R 0000 00ss ssst tttt dddd d000 0010 1010 slt   d s t
R 0000 00ss ssst tttt dddd d000 0010 1011 sltu  d s t
R iiii iiii iiii iiii iiii iiii iiii iiii .word i
```

R all start with 0000 00
J start with 0000 XX
I start with non-0000
