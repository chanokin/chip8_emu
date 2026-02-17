- [x] Instructions
  - [x] 0x0
    - [x] 0x00E0 Clear screen
    - [x] 0x00EE Return from subroutine
    - [x] 0x0NNN Call machine code routine at NNN

  - [x] 0x1
    - [x] 0x1NNN Goto NNN

  - [x] 0x2
    - [x] 0x2NNN Call subroutine at NNN

  - [x] 0x3
    - [x] 0x3XNN Skip the next instruction if value in Reg X is Equal to NN

  - [x] 0x4
    - [x] 0x4XNN Skip the next instruction if value in Reg X is Not Equal to NN

  - [x] 0x5
    - [x] 0x5XY0 Skip the next instruction if the values in Regs X and Y are Equal

  - [x] 0x6
    - [x] 0x6XNN Set the value of Reg X to number NN

  - [x] 0x7
    - [x] 0x7XNN Add NN to the value in Reg X (carry flag not altered)

  - [x] 0x8
    - [x] 0x8XY0 Set value of Reg X with the value of Reg Y
    - [x] 0x8XY1 Set value of Reg X with the bitwise OR of the values in Regs X and Y [V(X) = V(X) | V(Y)]
    - [x] 0x8XY2 Set value of Reg X with the bitwise AND of the values in Regs X and Y [V(X) = V(X) & V(Y)]
    - [x] 0x8XY3 Set value of Reg X with the bitwise XOR of the values in Regs X and Y [V(X) = V(X) ^ V(Y)]
    - [x] 0x8XY4 Set value of Reg X with the Addition of the values in Regs X and Y [V(X) = V(X) + V(Y)] Set VF to 1 on overflow, otherwise to 0
    - [x] 0x8XY5 Set value of Reg X with the Subtraction of the values in Regs X and Y [V(X) = V(X) - V(Y)] Set VF to 0 if underflow, otherwise to 1
    - [x] 0x8XY6 Right shift by one the value of Reg X and store the LSB of value (prior to shift) to VF
    - [x] 0x8XY7 Set value of Reg X with the Subtraction of the values in Regs Y and X [V(X) = V(Y) - V(X)] Set VF to 0 if underflow, otherwise to 1
    - [x] 0x8XYE Left shift by one the value of Reg X and store the MSB of value (prior to shift) to VF

  - [x] 0x9
    - [x] 0x9XY0 Skip next instruction if the values of Regs X and Y are Not Equal

  - [x] 0xA
    - [x] 0xANNN Set the Index Reg to the address/value NNN

  - [x] 0xB
    - [x] 0xBNNN Jump to address NNN plus V0

  - [x] 0xC
    - [x] 0xCXNN Set the value of Reg X to a random number between 0 and 255 (inclusive), and mask it with NN

  - [x] 0xD
    - [x] 0xDXYN Draw a sprite at coordinate [V(X), V(Y)] (values of Regs X, Y). The sprite has an 8 pixels width and an N pixels height.

  - [x] 0xE
    - [x] 0xEX9E Skip instruction if the key stored in Reg X is being pressed.
    - [x] 0xEXA1 Skip instruction if the key stored in Reg X is NOT being pressed.

  - [x] 0xF
    - [x] 0xFX07 Set the value of Reg X to the Delay timer value.
    - [x] 0xFX0A Set the value of Reg X to the value of the key pressed [blocking!].
    - [x] 0xFX15 Set Delay timer to the value of Reg X
    - [x] 0xFX18 Set Sound timer to the value of Reg X
    - [x] 0xFX1E Add the value of Reg X to the Index Reg. [VF not affected!]
    - [x] 0xFX29 Set the Index Reg to the location of the sprite for the character in Reg X
    - [x] 0xFX33 Store the BCD of the value in Reg X into the Index Reg I(+0,+1,+2) = BCD(VX|3,2,1)
    - [x] 0xFX55 Dump registers from 0 to X (inclusive) into the Index Reg I(+0,...,+X) = V(0,...,X)
    - [x] 0xFX65 Load the Index Reg into registers from 0 to X (inclusive)  V(0,...,X) = I(+0,...,+X)

