using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

public class WeatherForecast
{
    public DateTime Date { get; set; }
    public int TemperatureC { get; set; }
    public string Summary { get; set; }
    public int TemperatureF => 32 + (int)(TemperatureC / 0.5556);
}

public class WeatherService
{
    private static readonly string[] Summaries = new[]
    {
        "Freezing", "Bracing", "Chilly", "Cool", "Mild", "Warm", "Balmy", "Hot", "Sweltering", "Scorching"
    };

    public async Task<IEnumerable<WeatherForecast>> GetForecastAsync(int days)
    {
        var rng = new Random();
        await Task.Delay(1000); // Simulate API delay

        return Enumerable.Range(1, days).Select(index => new WeatherForecast
        {
            Date = DateTime.Now.AddDays(index),
            TemperatureC = rng.Next(-20, 55),
            Summary = Summaries[rng.Next(Summaries.Length)]
        }).ToArray();
    }
}

public class WeatherAnalyzer
{
    public static (double avgTemp, string mostCommonSummary) AnalyzeForecast(IEnumerable<WeatherForecast> forecasts)
    {
        var avgTemp = forecasts.Average(f => f.TemperatureC);
        var mostCommonSummary = forecasts
            .GroupBy(f => f.Summary)
            .OrderByDescending(g => g.Count())
            .First()
            .Key;

        return (avgTemp, mostCommonSummary);
    }
}

class Program
{
    static async Task Main()
    {
        var weatherService = new WeatherService();
        var forecasts = await weatherService.GetForecastAsync(5);

        Console.WriteLine("5-Day Weather Forecast:");
        foreach (var forecast in forecasts)
        {
            Console.WriteLine($"Date: {forecast.Date:d}, Temp: {forecast.TemperatureC}°C ({forecast.TemperatureF}°F), Summary: {forecast.Summary}");
        }

        var (avgTemp, mostCommonSummary) = WeatherAnalyzer.AnalyzeForecast(forecasts);
        Console.WriteLine($"\nAverage Temperature: {avgTemp:F1}°C");
        Console.WriteLine($"Most Common Weather: {mostCommonSummary}");
    }
}
