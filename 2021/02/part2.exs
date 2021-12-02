defmodule Answer do
  defstruct [:horizontal, :depth, :aim, :answer]

  def run() do
    File.read!("./input.txt")
    |> parse()
    |> solve(0, 0, 0)
    |> IO.inspect()
  end

  def parse(input) do
    input
    |> String.split("\n")
    |> Enum.map(&String.split(&1, " "))
    |> Enum.flat_map(fn
      [""] -> []
      ["up", n] -> [{:up, n}]
      ["down", n] -> [{:down, n}]
      ["forward", n] -> [{:forward, n}]
    end)
    |> Enum.map(fn {dir, n} -> {dir, String.to_integer(n)} end)
  end

  def solve([], horizontal, depth, aim) do
    %Answer{horizontal: horizontal, depth: depth, aim: aim, answer: horizontal * depth}
  end

  def solve([{:up, n} | rest], horizontal, depth, aim) do
    solve(rest, horizontal, depth, aim - n)
  end

  def solve([{:down, n} | rest], horizontal, depth, aim) do
    solve(rest, horizontal, depth, aim + n)
  end

  def solve([{:forward, n} | rest], horizontal, depth, aim) do
    solve(rest, horizontal + n, depth + aim * n, aim)
  end
end

Answer.run()
