async function main() {
  const graph = new Graph
  const links = await getInput()

  graph.register(links)

  const walker = new Walker([], true)
  const routes = walker.routesFrom(graph, 'start', 'end')

  console.log(`Answer: ${routes.length}`)
}


type Route = string[]


class Walker {
  visited: Set<string>
  haveTime: boolean
  trail: Route

  constructor(trail: Route, haveTime: boolean, visited: Set<string>|null = null) {
    this.visited = visited ?? new Set
    this.haveTime = haveTime
    this.trail = trail
  }

  routesFrom(graph: Graph, start: string, end: string): Route[] {
    if (start === end) return [[...this.trail, 'end']]

    const node = graph.getOrFail(start)

    if (! node.big) {
      this.visited.add(start)
    }
    this.trail.push(start)

    return node?.links.map(link => {
      if (this.visited.has(link)) {
        if (this.haveTime && link !== 'start' && ! Node.isBig(link)) {
          return this.cloneForExtraVisit(link).routesFrom(graph, link, end)
        }
        return []
      }
      return this.clone().routesFrom(graph, link, end)
    }).flat(1)
  }

  clone(): Walker {
    return new Walker(
      [...this.trail],
      this.haveTime,
      new Set(this.visited),
    )
  }

  cloneForExtraVisit(name: string): Walker {
    const visited = new Set(this.visited)
    visited.delete(name)
    return new Walker([...this.trail], false, visited)
  }
}


class Node {
  name: string
  big: boolean
  links: string[] = []

  constructor(name: string) {
    this.name = name
    this.big = Node.isBig(name)
  }

  link(otherName: string): void {
    this.links.push(otherName)
  }

  static isBig(name: string) {
    return name[0].toUpperCase() === name[0]
  }
}


class Graph extends Map<string, Node> {
  getOrFail(name: string): Node {
    const node = this.get(name)
    if (! node) throw new Error(`node ${name} not on the Graph`)
    return node
  }

  getOrAdd(name: string): Node {
    let node = this.get(name)
    if (! node) {
      node = new Node(name)
      this.set(name, node)
    }
    return node
  }

  register(links: Input): void {
    links.forEach(link => {
      const from = this.getOrAdd(link[0])
      const to = this.getOrAdd(link[1])
      from.link(link[1])
      to.link(link[0])
    })
  }
}


type Input = [string, string][]


async function getInput(): Promise<Input> {
  const input = await Deno.readTextFile('input.txt')
  return input.trim().split('\n').map(item => {
    const [from, to] = item.split('-')
    return [from, to]
  })
}

main()
