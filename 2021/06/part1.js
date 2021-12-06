fs = require('fs')

const DAYS = 80

fs.readFile('./input.txt', 'utf8', main)

function main(err, data) {
  let school = data.trim().split(',').map(n => new Fish(n))

  for (let day = 1; day <= DAYS; day++) {
    console.log(`Day ${day}`)
    const population = copy(school.length)

    for (let i = 0; i < population; i++) {
      const fish = school[i]
      const child = fish.anotherDayInTheLife()
      if (child) {
        school.push(child)
      }
    }
    console.log(`There are ${school.length} more fish in the sea.`)
  }
}

class Fish {
  constructor(reproductionUrge) {
    this.reproductionUrge = Number(reproductionUrge)
  }

  anotherDayInTheLife() {
    this.reproductionUrge--
    if (this.reproductionUrge < 0) {
      this.reproductionUrge = 6
      return new Fish(8)
    }
  }
}

function copy(thing) {
  return JSON.parse(JSON.stringify(thing))
}
