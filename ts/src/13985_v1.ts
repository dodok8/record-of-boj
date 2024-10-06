const fs = require('fs')

function* inputIterator<T>(arr: Array<T>) {
  for (const num of arr) {
    yield num
  }
}

function main() {
  let input_path = '/dev/stdin'
  try {
    if (process.env.LOCAL) {
      input_path = '/workspaces/record-of-boj/input/13985_v1'
    }
  } catch {}
  const input = inputIterator(
    fs.readFileSync(input_path).toString().split(/\s/) as string[]
  )

  const a = Number(input.next().value)
  input.next()
  const b = Number(input.next().value)
  input.next()
  const c = Number(input.next().value)

  console.log(a + b == c ? 'YES' : 'NO')
}

main()
