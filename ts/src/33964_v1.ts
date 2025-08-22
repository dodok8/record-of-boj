// 레퓨닛의 덧셈

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

  const x = Number(input.next().value)
  const y = Number(input.next().value)

  
  const repunitX = parseInt("1".repeat(x))
  const repunitY = parseInt("1".repeat(y))

  console.log(repunitX + repunitY)
}

main()
