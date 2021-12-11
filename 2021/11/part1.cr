chart = get_input()
STEPS = 100
SLEEP = 0.2
CHART_SIZE = 10

flashes = 0

(1..STEPS).each do |step|
  flashes += apply_step(step, chart)
  puts "Step #{step}, #{flashes} flashes"
  print_chart(chart)
  if step != STEPS
    cursor_back(CHART_SIZE + 1)
  end
end
puts "Answer:   #{flashes} flashes"


def apply_step(step, chart)
  chart.each_with_index do |row, y|
    row.each_with_index do |col, x|
      charge(step, chart, x, y)
    end
  end
  flashes = 0
  chart.each_with_index do |row, y|
    row.each_with_index do |col, x|
      if chart[y][x][:flashed?]
        flashes += 1
        chart[y][x] = {charge: 0, flashed?: false}
      end
    end
  end
  flashes
end

def charge(step, chart, x, y)
  octo = chart[y][x]
  charge = octo[:charge]
  charge += 1
  if charge > 9 && ! octo[:flashed?]
    chart[y][x] = {charge: charge, flashed?: true}

    neighbors(x, y) do |x, y|
      charge(step, chart, x, y)
    end
  else
    chart[y][x] = {charge: charge, flashed?: octo[:flashed?]}
  end
end

def neighbors(x, y, &block)
  if x - 1 >= 0 && y - 1 >= 0
    yield x - 1, y - 1
  end
  if x - 1 >= 0
    yield x - 1, y
  end
  if x - 1 >= 0 && y + 1 < CHART_SIZE
    yield x - 1, y + 1
  end

  if y - 1 >= 0
    yield x, y - 1
  end
  # if :self
  #   yield x, y
  # end
  if y + 1 < CHART_SIZE
    yield x, y + 1
  end

  if x + 1 < CHART_SIZE && y - 1 >= 0
    yield x + 1, y - 1
  end
  if x + 1 < CHART_SIZE
    yield x + 1, y
  end
  if x + 1 < CHART_SIZE && y + 1 < CHART_SIZE
    yield x + 1, y + 1
  end
end

def get_input()
  File.read_lines("input.txt").map do |line|
    numbers = [] of NamedTuple(charge: Int32, flashed?: Bool)
    line.each_char do |char|
      numbers << {charge: char.to_i32, flashed?: false}
    end
    numbers
  end
end

def print_chart(chart)
  chart.each do |row|
    row.each do |col|
      charge = col[:charge]
      if charge == 0
        print " \e[1m\e[33m0\e[0m"
      else
        print " #{charge}"
      end
    end
    print "\n"
  end
  sleep(SLEEP)
end

def cursor_back(times)
  print "\e[#{times}A"
end
