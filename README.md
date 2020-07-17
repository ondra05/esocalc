esocalc
=======
*Esoteric, yet usable calculator*

## How to use?
Esocalc is working with expressions and subpressions

### Concepts
- Expression = Sum of all subpressions **-> result**
- Subpression = You can enter here input variables and operations
¨
### Symbols
- **%**: Number
- **^**: Power

### Operands
Classic +, -, * and /

You add to symbol this way:
```
%+2
```
will add 2 to **%** symbol
```
%+3^+2
```
will add 3 to **%** and 2 to **^**

### How to eval
To eval subpression, just add @ or &

- @: End of subpression, reset % and ^
- &: End of subpression, keep % and ^
- ;: End of **expression**, sum all subpressions and print

```
%+3^+2@%+2^+2;
```
**%+3^+2**: Add 3 to **%** and 2 to **^**

**@**: Power 3 by 2 (3^2 = 9) and add to result

**%+2^+2**: Add 2 to **%** and 2 to **^**

**;**: Power 2 by 2 (2^2 = 4), add to result and print

**-> 13**

## Conditions
Conditions starts with **?**
```
?(EXPRESSION, EXPECTED RESULT)(EXPRESSION TO DO IF TRUE)
```
for example:
```
?(%+3^+2;,9)(%+4^+2;)
```
%+3^+2; == 9

9 == 9 -> TRUE

Do %+4^+2;

**-> 16**

You can invert it by adding ! after ?
```
?!(%+3^+2;,1)(%+4^+2;)
```
**-> 16**

## Internal commands
To quit REPL: quit, exit, q

To print version: version, ver

## Credits
Created by *ondra05*

Licensed under [MIT License](LICENSE)