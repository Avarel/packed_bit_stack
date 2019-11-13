Packed Bit Stack
===

An experimental stack datastructure that stores both numeric values and heap objects with zero padding and zero guarantees (😉 know what you're doing).

This crate stores numbers of all sizes continuously next to each other, with no padding. When you want to read a number, be sure to specify the type of number that you want to pop.

In addition, miscellaneous values are also stored in two styles: heap leaking and raw bits. In heap leaking, objects are leaked from the borrow checker and their pointers are stored in the stack. If they are not popped and dropped, their memory will leak. This is why the functions for storing and popping them are unsafe. In raw bits, the objects bits are taken and the original objects are destroyed by dropping them into hell. The bits are now live in the wastelands, ready to be popped into life again. Both of them may leak if something goes wrong, so have fun.