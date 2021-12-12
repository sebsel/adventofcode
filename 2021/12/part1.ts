async function main() {
  const graph = new Graph
  const links = await getInput()

  graph.register(links)

  const walker = new Walker([])
  const routes = walker.routesFrom(graph, 'start', 'end')

  console.log(routes.map(route => route.join(',')))
  console.log(`Answer: ${routes.length}`)
}


type Route = string[]


class Walker {
  visited: Set<string>
  trail: Route

  constructor(trail: Route, visited: Set<string> | null = null) {
    this.visited = visited ?? new Set
    this.trail = trail
  }

  routesFrom(graph: Graph, start: string, end: string): Route[] {
    if (start == end) return [[...this.trail, 'end']]

    const node = graph.getOrFail(start)

    if (! node.big) {
      this.visited.add(start)
    }
    this.trail.push(start)

    return node?.links.map(link => {
      if (this.visited.has(link)) {
        return []
      }
      const visited = new Set(this.visited)
      const walker = new Walker([...this.trail], visited)
      return walker.routesFrom(graph, link, end)
    }).flat(1)
  }
}


class Node {
  name: string
  big: boolean
  links: string[] = []

  constructor(name: string) {
    this.name = name
    this.big = name[0].toUpperCase() === name[0]
  }

  link(otherName: string): void {
    this.links.push(otherName)
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
