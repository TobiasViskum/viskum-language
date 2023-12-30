let a = "global a";
let b = "global b";
let c = "global c";
{
  let a = "outer a";
  let b = "outer b";
  {
    let a = "inner a";
    c = "This was modified";
    print a; // inner a
    print b; // outer b
    print c; // global c
  }
  print a; // outer a
  print b; // outer b
  print c; // global c
}
print a; // global a
print b; // global b
print c; // global c

/*
if 1 + 1 == 2 ? "this is cool or" : "not"

let a = 4, b = 2, c = true;

let number = match "32".to_number() {
    Ok(v) => v,
    Err(e) => {
        print("Could not convert str to num");
        32
    }
}

"hejsa2".to_number()
*/
