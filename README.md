Simple Chip8 emulator written in Rust, mainly to teach myself the language.

It's quite barebone, it opens a simple window and render the frame buffer;
you can then
- open a ROM by pressing the _O_ key
- reset (full clear) by pressing the _I_ key

and the keyboard is mapped this way
```
 1 2 3 C                1 2 3 4
 4 5 6 D    <-------    Q W E R
 7 8 9 E                A S D F
 A 0 B F                Z X C V
```


Test roms:

https://github.com/corax89/chip8-test-rom/tree/master

https://github.com/loktar00/chip8/tree/master/roms

https://github.com/Timendus/chip8-test-suite/


Chip8 info:

https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

https://en.wikipedia.org/wiki/CHIP-8

https://chip-8.github.io/links/