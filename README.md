# The Viskum programming language

An interpreter (and maybe compiler?) for the Viskum Programming Language

I'm following this book: https://craftinginterpreters.com/contents.html. I'm writing it in Rust instead of Java.

## Syntax

### Variable declaration

<code>let a = 2</code>

If you want the variable to mutable, you follow the Rust syntax (not implemented yet. For now all variables are mutable)

<code>let mut a = 2</code>

### Extended mathematical operations

<code>5! = 5 \* 4 \* 3 \* 2 \* 1</code>
<br/>
<code>5^2 = 25</code>

And then there are all the standard operations as well, that you see in most programming languages.

### Ternary operator

<code>1 + 2 == 3 ? "This was true" : "This was false"</code>

### Comments

Single line comments:
<code>// This is a comment</code>

Multi line comments:
<code>/\*

This is a

multi line comment

\*/ </code>
