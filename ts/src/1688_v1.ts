// 지민이의 테러

const fs = require("fs");

type Vector = [number, number];
let polygon: Vector[] = [];

function getCCW(p1: Vector, p2: Vector, p3: Vector) {
  const [x1, y1] = p1;
  const [x2, y2] = p2;
  const [x3, y3] = p3;
  return x1 * y2 + x2 * y3 + x3 * y1 - (x2 * y1 + x3 * y2 + x1 * y3);
}

function getPolygonInside(point: Vector): boolean {
  let cntCrossed = 0;
  for (let idx = 0; idx < polygon.length; idx++) {
    const jdx = (idx + 1) % polygon.length;
    if (
      getCCW(polygon[idx], polygon[jdx], point) == 0 &&
      ((point[0] > polygon[idx][0]) != (point[0] > polygon[jdx][0]))
    ) {
      return true;
    }

    if ((point[1] > polygon[idx][1]) != (point[1] > polygon[jdx][1])) {
      let crossedX = (polygon[jdx][0] - polygon[idx][0]) *
          (point[1] - polygon[idx][1]) / (polygon[jdx][1] - polygon[idx][1]) +
        polygon[idx][0];

      if (crossedX > point[0]) {
        cntCrossed += 1;
      }
    }
  }

  return cntCrossed % 2 != 0;
}

function* inputIterator<T>(arr: Array<T>) {
  for (const num of arr) {
    yield num;
  }
}

function main() {
  let input_path = "/dev/stdin";
  let input = inputIterator(
    fs
      .readFileSync(input_path)
      .toString()
      .split(/\s/)
      .map((x: string) => parseInt(x)),
  );

  let num_v = input.next().value as number;

  for (let _idx = 0; _idx < num_v; _idx++) {
    let x = input.next().value as number;
    let y = input.next().value as number;

    polygon.push([x, y]);
  }

  for (let _ = 0; _ < 3; _++) {
    let x = input.next().value as number;
    let y = input.next().value as number;

    if (getPolygonInside([x, y])) {
      console.log(1);
    } else {
      console.log(0);
    }
  }
}

main();
