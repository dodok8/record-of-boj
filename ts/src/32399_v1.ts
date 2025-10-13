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

  if (target === ")1(") {
    console.log(2);
  } else if (target === "(1)") {
    console.log(0);
  } else {
    console.log(1);
  }
}

main();
