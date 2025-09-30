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

  let lastWinner = "D";
  let cntA = 0;
  let cntB = 0;

  const scoresA: number[] = (await readLine()).split(" ").map((
    score: string,
  ) => parseInt(score, 10));
  const scoresB: number[] = (await readLine()).split(" ").map((
    score: string,
  ) => parseInt(score, 10));

  for (const idx of range(0, 10)) {
    if (scoresA[idx] > scoresB[idx]) {
      cntA += 3;
      lastWinner = "A";
    } else if (scoresA[idx] < scoresB[idx]) {
      cntB += 3;
      lastWinner = "B";
    } else {
      cntA += 1;
      cntB += 1;
    }
  }

  console.log(`${cntA} ${cntB}`);
  if (cntA > cntB) {
    console.log("A");
  } else if (cntA < cntB) {
    console.log("B");
  } else {
    console.log(lastWinner);
  }
}

main();
