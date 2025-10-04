import readline from "readline";

function* range(start: number, end: number) {
  for (let i = start; i < end; i++) {
    yield i;
  }
}

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

  const n = parseInt(await readLine(), 10);

  let bestIdx = 0;
  let bestTime = 0;
  let maxScore = -1;

  for (const idx of range(0, n)) {
    const line = await readLine();
    const [t, s] = line.split(" ").map(Number);

    if (maxScore < s) {
      bestIdx = idx + 1;
      bestTime = t;
      maxScore = s;
    }
  }

  if (maxScore === 0) {
    console.log(0);
  } else {
    console.log(bestTime + (bestIdx - 1) * 20);
  }
}

main();
