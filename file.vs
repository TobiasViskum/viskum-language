fn fib(n) {
  if n <= 1 {
    return n;
  }
  return fib(n - 2) + fib(n - 1);
}

let i = 0;
while i < 30 {
  i = i + 1;
  print fib(i);
}



/*

let start_time = time();
let a = 8!;
let b = 16!;
let c = 24!;

let result = "No match";

if 1 + 2 == 2 {
  result = "first";
} else if 2 + 2 == 4 {
  result = "second";
} else {
  result = "third";
}


print "string";


let count = 0;

while count < 10 {
  count = count + 1;
  print count;
  continue;
}

print "Finished";



1 + 1 == 2 ? "this is cool or" : "not";

print time() - start_time;

*/