const fs = require("fs");

function main() {
  function* inputIterator<T>(arr: Array<T>) {
    for (const num of arr) {
      yield num;
    }
  }
  let input_path = "/dev/stdin";
  try {
    if (process.env.LOCAL) {
      input_path = "/workspaces/record-of-boj/input/31611_v1";
    }
  } catch {}
  let input = inputIterator(
    fs
      .readFileSync(input_path)
      .toString()
      .split(/\s/)
      .map((x: any) => Number(x)),
  );
  let x = input.next().value as number;

  console.log(x % 7 == 2 ? "1" : "0");
}

main();
