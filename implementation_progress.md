- [ ] Instructions
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

  - [ ] 0x9

  - [ ] 0xA

  - [ ] 0xB

  - [ ] 0xC

  - [ ] 0xD

  - [ ] 0xE

  - [ ] 0xF
