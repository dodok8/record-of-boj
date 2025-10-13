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

  const [a, b] = (await readLine()).split(" ").map((x: string) =>
    parseInt(x, 10)
  );
  const [c, d] = (await readLine()).split(" ").map((x: string) =>
    parseInt(x, 10)
  );

  if (a + c < b + d) {
    console.log("Hanyang Univ.");
  } else if (a + c > b + d) {
    console.log("Yongdap");
  } else {
    console.log("Either");
  }
}

main();
