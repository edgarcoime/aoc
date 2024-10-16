defmodule Day1 do
  def read_file(filename) do
    path = Path.join(:code.priv_dir(:aoc), filename)

    # Read everything in text
    case File.read(path) do
      {:ok, content} -> IO.puts("File content #{content}")
      {:error, reason} -> IO.puts("Error reading file: #{reason}")
    end
  end

  @spec concatenate(integer, integer) :: integer
  def concatenate(a, b) do
    (a * :math.pow(10, 1) + b)
    |> round()
  end

  @spec process_line(String.t()) :: integer
  def process_line(line) do
    res =
      line
      |> String.graphemes()
      |> Enum.reduce(%{:prev => nil, :first => nil}, fn char, acc ->
        case Integer.parse(char) do
          {n, _} ->
            acc =
              if acc[:first] == nil do
                Map.put(acc, :first, n)
              else
                acc
              end

            acc = Map.put(acc, :prev, n)
            acc

          :error ->
            acc
        end
      end)

    # assuming always :prev and :first will not be nil
    concatenate(res[:first], res[:prev])
  end

  @spec read_file_by_line(String.t()) :: integer
  def read_file_by_line(filename) do
    path = Path.join(:code.priv_dir(:aoc), filename)

    try do
      path
      |> File.stream!()
      |> Enum.map(&process_line/1)
      |> Enum.reduce(0, &(&1 + &2))
    rescue
      error -> IO.puts("Error reading file: #{inspect(error)}")
    end
  end
end

# Pseudo code
# def day1():
#   path = filename
#   f = open(path)
#   total = 0
#   for line in file(f):
#     first = None
#     curr = None
#     prev = None
#     for ch in line:
#       # If not a digit skip
#       if not int(ch):
#         continue 
#
#       # if first is none then first digit
#       curr = int(ch)
#       if first is None:
#         first = curr
#
#       prev = curr
#
#     digit = int(concatenate(first, prev))
#     total += digit
