let iteration_count = 30;

fn fib(n) {
  if n <= 1 {
    return n;
  }
  return fib(n - 2) + fib(n - 1);
}

let i = 0;
while i < iteration_count {
  i = i + 1;
  print fib(i);
}

// iteration_count = 20: 0.04s to 0.05s
// iteration_count = 25: 0.37s
// iteration_count = 30: 3.85s to 3.86s