.origin 00 00

brk 7 'start

'data
.b Hello, World!\x0A\x00

'start
    puth 1 'data
    putl 1 'data

    lea d 01
    brk 7 'print
    trap 0 02
    halt

'print
    puth 2 00
    putl 2 01

'print_loop
    ld 0 01
    rs 0 08
    cmp 0
    brk 2 de

    ld 0 01
    trap 0

    ls 0 08
    cmp 0
    brk 2 de

    trap 0
    add 1 12
    brk 7 'print_loop
