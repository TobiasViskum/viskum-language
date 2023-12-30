# The Viskum programming language

An interpreter (and maybe compiler?) for the Viskum Programming Language

I'm following this book: https://craftinginterpreters.com/contents.html. I'm writing it in Rust instead of Java.

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

```
5! = 5 \* 4 \* 3 \* 2 \* 1
```

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
\*
This is a
block comment
*/
```

### If statements

<code></code>
