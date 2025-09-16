// 소수

const fs = require("fs");

function* inputIterator<T>(arr: Array<T>) {
  for (const num of arr) {
    yield num;
  }
}

function main() {
  let input_path = "/dev/stdin";
  let input = inputIterator(
    fs
      .readFileSync(input_path)
      .toString()
      .split(/\s/)
      .map((x: any) => Number(x)),
  );

  let a = input.next().value as number;
  let b = input.next().value as number;
  let n = input.next().value as number;

  if (a % b === 0) {
    console.log(0);
  } else if (a > b) {
    a = a % b;
  }

  let ans = 0;
  for (let idx = 0; idx < n; idx += 1) {
    a *= 10;
    ans = Math.trunc(a / b);
    a = a % b;
  }

  console.log(ans);
}

main();
