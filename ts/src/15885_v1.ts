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

  const [a, b]: [number, number] = (await readLine()).split(" ").map((
    x: string,
  ) => parseInt(x, 10));

  console.log(Math.floor(Math.abs(a / b - 1) * 2));
}

main();
