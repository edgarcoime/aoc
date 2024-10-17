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
      {color, String.to_integer(num)}
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
    # True or False
    IO.puts("Game: #{inspect(game)}")
    [game[:num], true]
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
    |> Enum.reduce(0, fn game, acc ->
      case game do
        [n, true] -> acc + n
        _ -> acc
      end
    end)
  end
end
