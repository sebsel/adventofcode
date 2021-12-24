defmodule Parser do
  @doc """
  Read in a hexadecimal code and parse it.
  """
  def digest_hex(code) do
    code
    |> Base.decode16!()
    |> to_bitlist([])
    |> parse(:start, 1, [])
    |> elem(0)
    |> hd()
    |> execute()
  end

  @doc """
  Turns a binary into a list of single bits.
  """
  def to_bitlist(<<>>, list), do: Enum.reverse(list)
  def to_bitlist(<<1::1, rest::bitstring>>, list), do: to_bitlist(rest, [1 | list])
  def to_bitlist(<<0::1, rest::bitstring>>, list), do: to_bitlist(rest, [0 | list])

  @doc """
  Turns a REVERSED bitlist into a number.
  """
  def to_number(list), do: do_to_number(list, 0, 0)

  defp do_to_number([], _times, number), do: number

  defp do_to_number([bit | rest], times, number),
    do: do_to_number(rest, times + 1, number + bit * 2 ** times)

  @doc """
  Parses the bitlist following the protocol.
  """
  def parse(rest, _status, 0, result), do: {result, rest}
  def parse([], _status, _expected, result), do: {result, []}

  # Packet version
  def parse([_, _, _ | rest], :start, exp, result) do
    parse(rest, :packet, exp, result)
  end

  # Number
  def parse([1, 0, 0 | rest], :packet, exp, result) do
    parse(rest, :number, exp, [[] | result])
  end

  def parse([1, a, b, c, d | rest], :number, exp, [number | result]) do
    number = [d, c, b, a | number]
    parse(rest, :number, exp, [number | result])
  end

  def parse([0, a, b, c, d | rest], :number, exp, [number | result]) do
    number = to_number([d, c, b, a | number])
    parse(rest, :start, exp - 1, [number | result])
  end

  # All other operators
  def parse([a, b, c | rest], :packet, exp, result) do
    operator =
      case {a, b, c} do
        {0, 0, 0} -> :sum
        {0, 0, 1} -> :prod
        {0, 1, 0} -> :min
        {0, 1, 1} -> :max
        {1, 0, 1} -> :gt
        {1, 1, 0} -> :lt
        {1, 1, 1} -> :eq
      end

    parse(rest, {:operator, operator}, exp, result)
  end

  def parse([0 | rest], {:operator, operator}, exp, result) do
    {sub, rest} = take(15, [], rest)
    size = to_number(sub)
    {sub, rest} = take(size, [], rest)
    {subs, []} = parse(Enum.reverse(sub), :start, 999_999_999, [])
    parse(rest, :start, exp - 1, [[operator | Enum.reverse(subs)] | result])
  end

  def parse([1 | rest], {:operator, operator}, exp, result) do
    {sub, rest} = take(11, [], rest)
    size = to_number(sub)
    {subs, rest} = parse(rest, :start, size, [])
    parse(rest, :start, exp - 1, [[operator | Enum.reverse(subs)] | result])
  end

  @doc """
  Execute the parsed AST.
  """
  def execute([operation | args]) when is_atom(operation),
    do: execute(operation, Enum.map(args, &execute/1))

  def execute(literal), do: literal

  def execute(:sum, args), do: Enum.sum(args)
  def execute(:prod, args), do: Enum.product(args)
  def execute(:min, args), do: Enum.min(args)
  def execute(:max, args), do: Enum.max(args)

  def execute(operator, [a, b]) do
    case(operator) do
      :gt -> a > b
      :lt -> a < b
      :eq -> a == b
    end
    |> case do
      true -> 1
      false -> 0
    end
  end

  defp take(0, sub, rest), do: {sub, rest}
  defp take(count, sub, [item | rest]), do: take(count - 1, [item | sub], rest)
end

IO.puts("Examples:")

run = fn hex ->
  hex
  |> IO.inspect()
  |> Parser.digest_hex()
  |> IO.inspect()
end

run.("C200B40A82")
run.("04005AC33890")
run.("880086C3E88112")
run.("CE00C43D881120")
run.("D8005AC2A8F0")
run.("F600BC2D8F")
run.("9C005AC2F8F0")
run.("9C0141080250320F1802104A08")

IO.puts("\nAnswer:")

File.read!("input.txt")
|> String.trim()
|> Parser.digest_hex()
|> IO.inspect()
