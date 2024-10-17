defmodule Day1p2 do
  @number_map %{
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
  }

  @spec is_named_int(String.t()) :: {String.t(), integer} | nil
  def is_named_int(str) do
    case Enum.find(@number_map, nil, fn {k, _} -> String.starts_with?(str, k) end) do
      nil -> nil
      {k, v} -> {k, v}
    end
  end

  @spec index_substring(String.t(), String.t()) :: integer | nil
  def index_substring(string, target) do
    # https://stackoverflow.com/questions/35551072/how-to-find-index-of-a-substring
    case String.split(string, target, parts: 2) do
      [left, _] -> String.length(left)
      [_] -> nil
    end
  end

  @spec concatenate(integer, integer) :: integer
  def concatenate(a, b) do
    (a * :math.pow(10, 1) + b)
    |> round()
  end

  @spec process_string_int(String.t()) :: {{integer, integer}, {integer, integer}}
  def process_string_int(line) do
    vals =
      line
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.reduce(%{:prev => nil, :first => nil}, fn {char, idx}, acc ->
        case Integer.parse(char) do
          {n, _} ->
            acc =
              if acc[:first] == nil do
                Map.put(acc, :first, {n, idx})
              else
                acc
              end

            acc = Map.put(acc, :prev, {n, idx})
            acc

          :error ->
            acc
        end
      end)

    {vals[:first], vals[:prev]}
  end

  @spec sliding_window(map, String.t()) :: {{integer, integer}, {integer, integer}}
  def sliding_window(%{:first => f, :prev => p, :idx => _}, ""), do: {f, p}

  @spec sliding_window(map, String.t()) :: {{integer, integer}, {integer, integer}}
  def sliding_window(state, str) do
    idx = Map.get(state, :idx)
    new_state = Map.put(state, :idx, idx + 1)

    case is_named_int(str) do
      nil ->
        sliding_window(new_state, String.slice(str, 1..-1//1))

      {_, v} ->
        curr_val = {v, idx}

        new_state =
          if state[:first] == nil do
            Map.put(new_state, :first, curr_val)
          else
            new_state
          end

        new_state = Map.put(new_state, :prev, curr_val)
        sliding_window(new_state, String.slice(str, 1..-1//1))
    end
  end

  def process_string_word(line) do
    sliding_window(%{:first => nil, :prev => nil, :idx => 0}, line)
  end

  def process_line(line) do
    parsed = line |> String.downcase() |> String.trim()
    vals_int = process_string_int(parsed)
    vals_word = process_string_word(parsed)

    {l, r} =
      case {vals_int, vals_word} do
        {{nil, nil}, {nil, nil}} ->
          {0, 0}

        {{nil, nil}, {{left, _}, {right, _}}} ->
          {left, right}

        {{{left, _}, {right, _}}, {nil, nil}} ->
          {left, right}

        {v_int, v_word} ->
          {{i_left_v, i_left_idx}, {i_right_v, i_right_idx}} = v_int
          {{w_left_v, w_left_idx}, {w_right_v, w_right_idx}} = v_word

          left_num =
            if i_left_idx < w_left_idx do
              i_left_v
            else
              w_left_v
            end

          right_num =
            if i_right_idx < w_right_idx do
              w_right_v
            else
              i_right_v
            end

          {left_num, right_num}
      end

    IO.puts("#{line}L: #{l}, R: #{r}\n")
    concatenate(l, r)
  end

  def read_file_by_line(filename) do
    path = Path.join(:code.priv_dir(:aoc), filename)

    path
    |> File.stream!()
    |> Enum.map(&process_line/1)
    |> Enum.reduce(0, &(&1 + &2))
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
