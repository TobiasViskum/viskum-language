# The Viskum programming language

<b>I'm working on a faster version right here: https://github.com/TobiasViskum/viskum-bytecode (chapter 15+)</b>

An interpreter for the Viskum Programming Language

I'm following this book: https://craftinginterpreters.com/contents.html. I'm writing it in Rust instead of Java, adding additional features and using my own syntax.

## Syntax

<b>Notice:</b> Right now you have to end every expression with a semicolon. That's not how it's going to work in the future, but that's how it works for now.

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
a-- // a: 8
```

### Comparison

Equals: ==

Not equal: !=

Greater than: >

Greater than or equal to: >=

Less than: <

Less than or equal to: <=

### Extended mathematical operations

Factorial (5! = 5 \* 4 \* 3 \* 2 \* 1):

```
5! = 120
```

Raise a number to the power of x:

```
(-5)^2 = 25
-5^2 = -25 // This doesn't work for some reason (equals to 25). Will be fixed.
```

And then there are all the standard operations as well, that you see in most programming languages like multiplication, addition, subraction and division (root of x will be implemented in the future).

### Negation

```
!true = false
!0 = true
!5 = false
!null = true
```

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

You can break any loop using the "break" keyword.
You can continue to the next iteration of any loop by using the "continue" keyword.

While loop:

```
while some_expr {
    // Do something
}
```

Loop without condition:

```
loop {
    // Do something
    if some_expr {
        // Remember to break the loop
        break
    }
}
```

### Functions

Native functions:

```
time() // Gets the current time in seconds
```

User functions:

```
fn new_function(some_str, some_number) {
    print "Argument 1: " + some_str
    print 5 * some_number
}

new_function("Some string", 5)

/*
Prints:

"Argument 1: Some string"
25
*/

```

## Todo

- For loops
- Do more chapters in the book (I'm at chapter 11)
- Explicit types

## Advanced pre-runtime error checker (todo)

- Check for infinite loops (error)
- Check for for example number + string (error)
- Check for no return statement if it's expected (error)
- Check for non-mutable variables that's assigned to a new value (error)

- Check for unused variables (warning)

## Other

- Total lines in project: 3026 (wc -l (src|build)/\*\*/\*.rs)
