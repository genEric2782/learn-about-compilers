using System;
using System.Text.Json;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;

class Program
{
    static async Task Main()
    {


        string input = await Console.In.ReadToEndAsync();

        var options = new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true,
            AllowTrailingCommas = true,
            WriteIndented = true,
        };

        var tokens = JsonSerializer.Deserialize<List<Token>>(input, options);


        var parser = new Parser(tokens);
        var ast = parser.ParseExpression();

        // Console.WriteLine("This is it ------------------------------------");
        Console.WriteLine(JsonSerializer.Serialize(ast, options));
    }
}
