// 키 큰사람
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

  while (true) {
    const n = Number(input.next().value)
    if (n === 0) break

    let max_len = 0
    let people: string[] = []

    for (let idx = 0; idx < n; idx++) {
      const curr_person = input.next().value as string
      const curr_len = Number(input.next().value)

      if (curr_len > max_len) {
        people = [curr_person]
        max_len = curr_len
      } else if (curr_len == max_len) {
        people.push(curr_person)
      } else {
        continue
      }
    }

    console.log(people.join(' '))
  }
}

main()
