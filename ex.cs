using System;

namespace MyApp
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
            foreach (var arg in args)
            {
                Console.WriteLine($"Argument: {arg}");
            }
        }
    }
}
