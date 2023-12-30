# The Viskum programming language

An interpreter (and maybe compiler?) for the Viskum Programming Language

I'm following this book: https://craftinginterpreters.com/contents.html. I'm writing it in Rust instead of Java.

## Todo

- Loops (while, loop, for i, v in arr {}, for i = 0, i < 2, i++ {})
- Functions

## Syntax

### Variable declaration

```
let a = 2
```

If you want the variable to mutable, you follow the Rust syntax (not implemented yet. For now all variables are mutable)

```
let mut a = 2
```

### Extended mathematical operations

Factorial (5! = 5 \* 4 \* 3 \* 2 \* 1):

```
5! = 120
```

Raise a number to the power of x:

```
5^2 = 25
```

And then there are all the standard operations as well, that you see in most programming languages.

### Ternary operator

```
1 + 2 == 3 ? "This was true" : "This was false"
```

### Comments

Single line comments:

```
// This is a comment
```

Block comments:

```
/*

This is a
block comment

*/
```

### If statements

```
if some_expression {
    // Do some stuff
} else if another_expression {
    // Do something else
} else {
    // If none of the two other expressions were true
}
```

### Logical operators

And:

```
if expr_1 and expr_2 {
    // Do something if both expressions were true
}
```

Or:

```
if expr_1 or expr_2 {
    // Do something if just one of them were true
}
```
