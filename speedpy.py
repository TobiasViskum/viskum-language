def fib(n):
    if n <= 1:
        return n
    return fib(n - 2) + fib(n - 1)

i = 0
while i < 40:
    i = i + 1
    print(fib(i))
