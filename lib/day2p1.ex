defmodule Day2p1 do
  # === CREATE DATASET ===
  def get_game_num(line) do
    line
    |> String.split([" ", ":"], trim: true)
    |> Enum.at(1)
    |> String.to_integer()
  end

  def get_games(line) do
    line
    |> String.split(":", parts: 2, trim: true)
    |> Enum.at(1)
    |> String.trim()
    |> String.split([";"], trim: true)
    |> Enum.map(&parse_colors(&1))
  end

  def parse_colors(line) do
    line
    |> String.trim()
    |> String.split(", ", trim: true)
    |> Enum.map(fn part ->
      [num, color] = String.split(part, " ")
      {String.to_atom(color), String.to_integer(num)}
    end)
    |> Enum.into(%{})
  end

  def process_line(line) do
    game_num = get_game_num(line)
    games = get_games(line)
    %{num: game_num, games: games}
  end

  # === CREATE DATASET ===

  # === PROCESS DATASET ===
  def process_game_set(game, target) do
    valid =
      game[:games]
      |> Enum.map(fn set ->
        # Iterate through all targets
        target
        |> Enum.map(fn {k, v} ->
          set_count = Map.get(set, k, 0)
          target_count = v

          if set_count <= target_count do
            true
          else
            false
          end
        end)
        |> Enum.all?(fn valid -> valid == true end)
      end)
      |> Enum.all?(fn valid -> valid == true end)

    res = {game[:num], valid}
    res
  end

  # === PROCESS DATASET ===

  def get_dataset_from_file(filename) do
    path = Path.join(:code.priv_dir(:aoc), filename)

    path
    |> File.stream!()
    |> Enum.map(&process_line(&1))
  end

  def cube_conundrum(red, green, blue, filename \\ "day2p1_test.txt") do
    dataset = get_dataset_from_file(filename)
    target = %{red: red, green: green, blue: blue}

    dataset
    |> Enum.map(&process_game_set(&1, target))
    |> Enum.filter(fn {_, valid} -> valid == true end)
    # |> Enum.each(fn v -> IO.puts("Valid: #{inspect(v)}") end)
    |> Enum.reduce(0, fn {num, _}, acc -> acc + num end)
  end
end
