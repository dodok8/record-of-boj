// Tier and Rank

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

  const [n, t]: [number, number] = (await readLine()).split(" ").map((
    x: string,
  ) => parseInt(x, 10));

  let rest = n;
  const tiers: Record<string, number> = {};

  for (let idx = 0; idx < t; idx++) {
    let [tier, numStr]: [string, string] = (await readLine()).split(" ");
    if (numStr.includes("%")) {
      let num = parseInt(numStr.slice(0, numStr.length - 1));
      num = Math.floor((rest * num) / 100);
      tiers[tier] = num;
      rest -= num;
    } else {
      let num = parseInt(numStr, 10);
      const M = Math.min(rest, num);
      tiers[tier] = M;
      rest -= M;
    }
  }

  if (rest > 0) {
    console.log("Invalid System");
    return;
  }

  const target: string = await readLine();

  if ([1, 2, 3, 4].includes(parseInt(target[target.length - 1], 10))) {
    const subTier = parseInt(target[target.length - 1], 10);
    const targetTier = target.slice(0, target.length - 1);

    if (!tiers[targetTier]) {
      console.log("Liar");
      return;
    }

    let tierSize = tiers[targetTier];

    const subTiers: number[] = [];
    const subTierSize = Math.ceil(tierSize / 4);
    while (tierSize > 0) {
      if (tierSize >= subTierSize) {
        subTiers.push(subTierSize);
        tierSize -= subTierSize;
      } else {
        subTiers.push(tierSize);
        break;
      }
    }

    if (subTiers.length < subTier) {
      console.log("Liar");
      return;
    }

    let base = 0;
    const tierList = Object.entries(tiers);

    for (let i = 0; i < tierList.length; i++) {
      if (tierList[i][0] === targetTier) break;
      else base += tierList[i][1];
    }

    for (let i = 0; i < subTier - 1; i++) {
      base += subTiers[i];
    }

    console.log(base + 1, base + subTiers[subTier - 1]);
  } else {
    if (!tiers[target]) {
      console.log("Liar");
      return;
    }

    let base = 0;
    const tierList = Object.entries(tiers);

    for (let i = 0; i < tierList.length; i++) {
      if (tierList[i][0] === target) break;
      else base += tierList[i][1];
    }

    console.log(base + 1, base + tiers[target]);
  }

  rl.close();
}

main();
