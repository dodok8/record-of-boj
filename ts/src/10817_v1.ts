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

  const nums: [number, number, number] = (await readLine()).split(" ").map((
    x: string,
  ) => parseInt(x, 10));

  nums.sort((a, b) => a - b);

  console.log(nums[1]);
}

main();
