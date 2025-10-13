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

  const target = await readLine();
  const T = parseInt(await readLine(), 10);

  let ans = 0;
  for (let idx = 0; idx < T; idx += 1) {
    const input = await readLine();
    if (target === input) ans += 1;
  }

  console.log(ans);
}

main();
