using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Day8
{
    class Program
    {
        static void Main(string[] args)
        {
            string str = File.ReadAllText("./input.txt");
            Queue<int> data = new Queue<int>(str.Split(' ').Select(s => int.Parse(s)));

            Node n = Node.FromData(data, 0);

            int totalMetadata = n.MetadataSum();

            Console.WriteLine($"Part 1: {totalMetadata}");

            int value = n.Value();

            Console.WriteLine($"Part 2: {value}");
            Console.ReadKey(true);
        }
    }
}
