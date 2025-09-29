const fs = require("fs");

function* inputIterator<T>(arr: Array<T>) {
  for (const num of arr) {
    yield num;
  }
}

function main() {
  let input_path = "/dev/stdin";
  try {
    if (process.env.LOCAL) {
      input_path = "/workspaces/record-of-boj/input/34400_v1";
    }
  } catch {}
  let input = inputIterator(
    fs
      .readFileSync(input_path)
      .toString()
      .split(/\s/)
      .map((x: any) => Number(x)),
  );

  const T = input.next().value as number;

  for (let _idx = 0; _idx < T; _idx++) {
    let t = input.next().value as number;

    if (t % 25 < 17) {
      console.log("ONLINE");
    } else {
      console.log("OFFLINE");
    }
  }
}

main();
