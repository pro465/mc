.origin 00 00

jmp 0 'start

.b Hello, World!\x0A\x00

'start
   put 1 00
   put 2 03
   put 3 01
   cmp 3
   mv E 0A
   mv D 09
   if 5
   jmp 0 'print
   halt

'print
    put 3 01

'print_loop
    ld 1 20
    cmp 0
    if 2
    jmp 0 'print_end
    trap 0
    add 3 22
    cmp 2
    if 2
    add 3 11
    jmp 0 'print_loop

'print_end
    trap 2
    jmp 3 9A
