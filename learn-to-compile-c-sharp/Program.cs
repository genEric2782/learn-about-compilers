// TODO 

using System.Text.Json;

class Program
{
    static void Main()
    {
        string json = Console.In.ReadToEnd();
        var tokens = JsonSerializer.Deserialize<List<Token>>(jsonString);

        var parser = new Parser();
        var ast = parser.Parse(tokens);

        Console.WriteLine(JsonSerializer.Serialize(ast));
    }
}
