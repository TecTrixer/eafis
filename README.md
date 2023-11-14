# Name

EAFIS: Easy And Fast Instruction Set

# Memory

- 24-bit addresses -> 16MB memory
- little endian
- no alignment required, but all instructions operate on 32-bit

# Registers

- a, b, c, d - general purpose registers
- r: for overflow with arithmetic or remainder with division, ...
- sp: stack pointer
- o: offset register
- ip: instruction pointer

# Addressing modes

- constant: 32-bit constant
- direct address: value at given 24-bit address offset by o-register
- register: 3-bit register
- register address: value at address at 3-bit register address offset by o-register (o-register is being interpreted as signed 32-bit int)

# Instruction layout

## Num of operands:

- 0 -> only opcode
- 1 -> opcode + addressing mode + either register 2 or direct address or constant
- 2 -> opcode + register 1 + addressing mode + either register 2 or direct address or constant

2 operand instructions always take one register and another field which is being addressed using the given addressing mode

- 8-bits: opcode
- 3-bit: register for op1
- 2-bit: addressing mode
- 3-bit: register for op2
- 32-bit constant / 24-bit address


# Opcodes

## 0-operand codes

|Opcode|Mnemonic|Description|
|-|-|-|
|0x00|HLT|halts the execution of the program|
|0x01|NOP|does nothing|
|0x02|SYS|calls a syscall (with syscall code in register a)|
|0x03|RET|pops value off the stack and sets ip to that value|

## 1-operand codes

|Opcode|Mnemonic|Description|
|-|-|-|
|0x10|JMP|sets instruction pointer to op1|
|0x11|JEQ|jumps if R is 0|
|0x12|JNE|jumps if R is not 0|
|0x13|JLT|jumps if R < 0 (signed)|
|0x14|JLE|jumps if R <= 0 (signed)|
|0x15|JGT|jumps if R > 0 (signed)|
|0x16|JGE|jumps if R >= 0 (signed)|

|Opcode|Mnemonic|Description|
|-|-|-|
|0x20|CALL|pushes next instruction address, decreases stack pointer by 4 and jumps to given address|
|0x21|PUSH|pushes value to the stack and decreases stack pointer by 4|
|0x22|POP|retrieves value from the stack and increases stack pointer by 4|
|0x23|INC|increases value by 1|
|0x24|DEC|decreases value by 1|

## 2-operand codes

|Opcode|Mnemonic|Description|
|-|-|-|
|0x30|CMP|sets R to op1 - op2|
|0x31|ST|stores op1 in op2|
|0x32|LD|loads op2 into op1|

|Opcode|Mnemonic|Description|
|-|-|-|
|0x40|NOT|sets op1 to !op2|
|0x41|XOR|sets op1 to op1 xor op2|
|0x42|AND|sets op1 to op1 and op2|
|0x43|OR|sets op1 to op1 or op2|
|0x44|ADD|sets op1 to op1 + op2|
|0x45|SUB|sets op1 to op1 - op2|
|0x46|MUL|sets op1 to op1 * op2 and R to the overflow|
|0x47|DIV|sets op1 to op1 / op2 and R to the remainder|



## Order of execution

- R is being written to after the instruction
- instruction pointer is being increased before instruction is run
