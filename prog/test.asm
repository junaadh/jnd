; 0a01 0801 0003 0002 f005

Push 0x0a
Push #8
AddStack
PopRegister %a
Push 0x70
PopRegister %b
AddRegister %a %b
PushRegister %a

PopRegister %c
SubRegister %c %b
Push #2
PopRegister %a
AddRegister %pc %a
Push 0x9
Push 0xa
PopRegister %a

Interrupt f0
