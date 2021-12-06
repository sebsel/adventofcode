fs = require('fs')

const DAYS = 256

fs.readFile('./input.txt', 'utf8', main)

function main(err, data) {
  let school = data.trim().split(',').map(Number)
  let generations = [0, 1, 2, 3, 4, 5, 6, 7, 8]
    .map(gen => school.filter(fish => fish == gen).length)

  for (let day = 1; day <= DAYS; day++) {
    console.log(`Day ${day}`)

    const births = generations.shift()

    generations[6] += births
    generations[8] = births

    const total = generations.reduce((a, b) => a + b)

    console.log(`There are ${total} more fish in the sea.`)
  }
}
