.origin 00 00

jmp 0 'start

.b Hello, World!\x0A\x00

'start
   put 1 0
   put 2 3
   put 3 1
   cmp 3
   mv 14 10
   mv 13 9
   if 5
   jmp 0 'print
   halt

'print
    put 3 1

'print_loop
    ld 1 2 0
    cmp 0
    if 2
    jmp 0 'print_end
    trap 0
    add 3 2 2
    cmp 2
    if 2
    add 3 1 1
    jmp 0 'print_loop

'print_end
    trap 2
    jmp 3 9 10
