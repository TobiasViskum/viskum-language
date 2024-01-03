let a = 0;
let temp = 0;

let b = 1;
while a < 10000 {
  b = temp + b;

  print a;

  temp = a;
  a = b;
}