using System;
using System.Text.Json;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;

class Program
{
    static async Task Main()
    {

        // Start Process to Read lexed output from stdout 
        // Driver code for testing parser
        var psi = new ProcessStartInfo
        {
            FileName = "cargo",
            Arguments = "run --quiet",
            RedirectStandardOutput = true,
            UseShellExecute = false,
            CreateNoWindow = true,
            WorkingDirectory = @"/home/generic/compiler_project/learn-about-compilers/learn-to-compile"
        };

        var process = Process.Start(psi);

        string output = await process.StandardOutput.ReadToEndAsync();

        process.WaitForExit();

        var options = new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true,
            AllowTrailingCommas = true,
            WriteIndented = true,
        };

        var tokens = JsonSerializer.Deserialize<List<Token>>(output, options);


        var parser = new Parser(tokens);
        var ast = parser.ParseExpression();

        // Console.WriteLine("This is it ------------------------------------");
        Console.WriteLine(JsonSerializer.Serialize(ast, options));
    }
}
