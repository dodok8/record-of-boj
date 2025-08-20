// Base Conversion

const fs = require('fs')

function* inputIterator<T>(arr: Array<T>) {
  for (const num of arr) {
    yield num
  }
}

function main() {
  let input_path = '/dev/stdin'
  const input = inputIterator(
    fs.readFileSync(input_path).toString().split(/\s/) as string[]
  )

  const a = Number(input.next().value)
  const b = Number(input.next().value)
  const m = Number(input.next().value)

  let base10 = 0; 
  for (let idx = m - 1; idx >= 0; idx--){
    base10 += Number(input.next().value) * a ** idx;
  }

  let ans: number[] = []
  if (base10 == 0) {
    ans.push(0)
  }
 
  while (base10 != 0) {
    ans.push(base10 % b)
    base10 = Math.floor(base10 / b)
  }

  console.log(ans.reverse().join(' '))
}

main()
