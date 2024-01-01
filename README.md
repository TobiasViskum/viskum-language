# The Viskum programming language

An interpreter (and maybe compiler?) for the Viskum Programming Language

I'm following this book: https://craftinginterpreters.com/contents.html. I'm writing it in Rust instead of Java, adding additional features and using my own syntax.

## Syntax

### Variable declaration

```
let a = 2
```

If you want the variable to mutable, you follow the Rust syntax (not implemented yet. For now all variables are mutable)

```
let mut a = 2
```

### Variable assignment

Any variable can be set to a new value of the same type with the '=' operator:

```
let a = "initial value"
a = "new value"
```

Any variable of type number can also be assigned with the following operators:

```
let a = 5
a += 1 // a: 6
a -= 2 // a: 4
a *= 3 // a: 12
a /= 6 // a: 2
a ^= 3 // a: 8
a++ // a: 9
a-- // a:8
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

### Loops

While loop:

```
while some_expr {
    // Do something
}
```

## Todo

- Loops (loop, for value in arr {}, for i = 0; i < 2; i++ {})
- Functions
- Support for increment (++)
- Support for decrement (--)
- Support for increment and assign (+=)
- Support for decrement and assign (-=)
- Support for break keyword

Maybe support for these as well:

- variable \*= 2;
- variable /= 2;
- variable ^= 2;

## Advanced pre-runtime error checker (todo)

- Check for infinite loops (error)
- Check for for example number + string (error)
- Check for no return statement if it's expected (error)
- Check for non-mutable variables that's assigned to a new value (error)

- Check for unused variables (warning)
