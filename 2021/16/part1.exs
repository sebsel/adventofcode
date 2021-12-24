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
    |> sum(0)
  end

  defp sum([], sum), do: sum
  defp sum([item | rest], sum) when is_list(item), do: sum(rest, sum + sum(item, 0))
  defp sum([item | rest], sum), do: sum(rest, sum + item)

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
  def parse([a, b, c | rest], :start, exp, result) do
    parse(rest, :packet, exp, [to_number([c, b, a]) | result])
  end

  # Number
  def parse([1, 0, 0 | rest], :packet, exp, result) do
    parse(rest, :number, exp, result)
  end

  def parse([1, _, _, _, _ | rest], :number, exp, result) do
    parse(rest, :number, exp, result)
  end

  def parse([0, _, _, _, _ | rest], :number, exp, result) do
    parse(rest, :start, exp - 1, result)
  end

  # All other operators
  def parse([_, _, _ | rest], :packet, exp, result) do
    parse(rest, :operator, exp, result)
  end

  def parse([0 | rest], :operator, exp, result) do
    {sub, rest} = take(15, [], rest)
    size = to_number(sub)
    {sub, rest} = take(size, [], rest)
    {sub, []} = parse(Enum.reverse(sub), :start, 999_999_999, [])
    parse(rest, :start, exp - 1, [sub | result])
  end

  def parse([1 | rest], :operator, exp, result) do
    {sub, rest} = take(11, [], rest)
    size = to_number(sub)
    {subs, rest} = parse(rest, :start, size, [])
    parse(rest, :start, exp - 1, [Enum.reverse(subs) | result])
  end

  defp take(0, sub, rest), do: {sub, rest}
  defp take(count, sub, [item | rest]), do: take(count - 1, [item | sub], rest)
end

IO.puts("Examples:")

"8A004A801A8002F478"
|> IO.inspect()
|> Parser.digest_hex()
|> IO.inspect()

"620080001611562C8802118E34"
|> IO.inspect()
|> Parser.digest_hex()
|> IO.inspect()

"C0015000016115A2E0802F182340"
|> IO.inspect()
|> Parser.digest_hex()
|> IO.inspect()

"A0016C880162017C3686B18A3D4780"
|> IO.inspect()
|> Parser.digest_hex()
|> IO.inspect()

IO.puts("\nAnswer:")

File.read!("input.txt")
|> String.trim()
|> Parser.digest_hex()
|> IO.inspect()
