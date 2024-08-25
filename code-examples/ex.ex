defmodule TextProcessor do

  def clean_text(text) do
    text
    |> String.replace(~r/[^\w\s]/, "")
    |> String.downcase()
  end

  @doc """ Processes text to extract frequent word counts. """
  def process_text(text, min_frequency) do
    text
    |> clean_text()
    |> split_into_words()
    |> count_words()
    |> filter_frequent_words(min_frequency)
  end
end

# Example usage of the TextProcessor module
defmodule TextAnalyzer do
  def run do
    text = "Hello, world! Welcome to the world of Elixir. Elixir is great; so great, indeed!"
    frequent_words = TextProcessor.process_text(text, 2)

    IO.inspect(frequent_words)
  end
end

TextAnalyzer.run()
