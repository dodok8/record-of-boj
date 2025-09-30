const readline = require("readline");

function* range(start: number, end: number) {
  for (let i = start; i < end; i++) {
    yield i;
  }
}

async function main() {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false,
  });

  const iterator = rl[Symbol.asyncIterator]();

  const readLine = async () => {
    const { value } = await iterator.next();
    return value;
  };

  const T = parseInt(await readLine(), 10);
  for (const _tdx of range(0, T)) {
    const _N = parseInt(await readLine(), 10);
    console.log(
      (await readLine()).split(/\s/).map((x: string) => parseInt(x, 10))
        .reduce((a: number, b: number) => a + b),
    );
  }
}

main();
