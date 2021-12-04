def main
  reader = BingoReader.new("input.txt")
  boards = reader.boards

  for number in reader.numbers
    for board in boards
      board.mark(number)

      if board.bingo?
        found(board, number)
        return
      end
    end
  end
end

def found(board, number)
  sum = board.all_unmarked.reduce(&:+)
  puts "Answer: #{sum * number}"
end



class BingoReader
  attr_reader :numbers, :boards

  def initialize(file)
    file = File.read(file).split("\n")
    @numbers = parse_numbers(file[0])
    @boards = parse_boards(file[1..])
  end

  private
  def parse_numbers(line)
    line.split(",").map(&:to_i)
  end

  def parse_boards(lines)
    lines.reduce([]) do |boards, line|
      next boards << [] if line == ""
      boards[-1] << line.split(" ").map(&:to_i)
      boards
    end.map do |lines|
      BingoBoard.new(lines.flatten)
    end
  end
end



class BingoBoard
  SIZE = 5

  def initialize(numbers)
    @numbers = numbers
  end

  def mark(number)
    @numbers =
      @numbers.map do |n|
          next nil if n === number
          n
      end
  end

  def rows
    rows = Array.new(SIZE) { [] }
    @numbers.each_with_index do |number, index|
      rows[index.div SIZE] << number
    end
    rows
  end

  def columns
    columns = Array.new(SIZE) { [] }
    @numbers.each_with_index do |number, index|
      columns[index.modulo SIZE] << number
    end
    columns
  end

  def bingo?
    for row in rows
      return true if row_bingo? row
    end
    for column in columns
      return true if row_bingo? column
    end
    return false
  end

  def all_unmarked
    @numbers.filter { |number| number != nil}
  end

  private
  def row_bingo?(row)
    for n in row
      return false if n != nil
    end
    return true
  end
end



main()
