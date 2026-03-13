defmodule Day2p2 do
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
  def process_game_set(game) do
    # Power red * green * blue
    min_map = %{red: 0, green: 0, blue: 0}
    # Edge case: what if there is clue for a color of cube then it would be 0?
    #             if that's the case then the power would be 0 as well

    power =
      game[:games]
      |> List.foldl(min_map, fn set, acc ->
        # %{blue: 6, green: 1, red: 3},
        Enum.reduce(set, acc, fn {k, v}, local_min ->
          curr_cube_max = Map.get(local_min, k)

          if curr_cube_max < v do
            Map.put(local_min, k, v)
          else
            local_min
          end
        end)
      end)
      |> Enum.reduce(1, fn {_key, v}, acc -> acc * v end)

    # Return power of set of cubes
    IO.puts("Power: #{power}\nSets: #{inspect(game[:games])}")
    power
  end

  # === PROCESS DATASET ===

  def get_dataset_from_file(filename) do
    # p2 dataset does not change
    path = Path.join(:code.priv_dir(:aoc), filename)

    path
    |> File.stream!()
    |> Enum.map(&process_line(&1))
  end

  def cube_conundrum(filename \\ "day2p1_test.txt") do
    dataset = get_dataset_from_file(filename)

    dataset
    |> Enum.map(&process_game_set(&1))
    |> Enum.reduce(0, &(&1 + &2))
  end
end
