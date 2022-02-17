A simple calculator using integer numbers. Based on the original tutorial:
[Let's Build A Simple Interpreter](https://ruslanspivak.com/lsbasi-part1/).

Supported grammar:
```
factor : (PLUS | MINUS) factor | INTEGER | LPAREN expr RPAREN

term : factor ((MUL | DIV) factor)*

expr : term ((PLUS | MINUS) term)*
```

Sample output with debug info:
```
>> 4+(12*5 / (2 + 1))
[Debug] tokens read:
Integer(4)
Add
LParen
Integer(12)
Mul
Integer(5)
Div
LParen
Integer(2)
Add
Integer(1)
RParen
RParen
=24
```