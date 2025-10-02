import readline from "readline";

async function main() {
  const rl = readline.createInterface({
    input: process.stdin,
    terminal: false,
  });

  const iterator = rl[Symbol.asyncIterator]();

  const readLine = async () => {
    const { value } = await iterator.next();
    return value;
  };

  let idx = 1;
  while (true) {
    const n = parseInt(await readLine());

    if (n === 0) break;

    let ans = 0;
    let divisor = 5;
    while (true) {
      let curr = Math.floor(n / divisor);

      if (curr === 0) break;
      ans += curr;
      divisor *= 5;
    }

    console.log(`Case #${idx}: ${ans}`);
    idx += 1;
  }
}

main();
