using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Diagnostics;

namespace Day2
{
    class Program
    {
        static void Main(string[] args)
        {
            Stopwatch stp = new Stopwatch();

            stp.Start();
            Part1();
            Part2();
            stp.Stop();
            Console.WriteLine($"Elapsed: {stp.ElapsedMilliseconds}");
            Console.ReadKey(true);
        }

        private static void Part2()
        {
            char[][] rows = File.ReadAllText("./input.txt")
                .Split(Environment.NewLine)
                .Select(s => s.ToCharArray()).ToArray();

            for (int i = 0; i < rows.Length - 1; i++)
            {
                char[] row = rows[i];

                for (int j = i + 1; j < rows.Length; j++)
                {
                    char[] next = rows[j];

                    int differences = 0;

                    for(int k = 0; k < row.Length; k++)
                    {
                        if(row[k] != next[k])
                        {
                            differences++;
                        }
                    }

                    if(differences == 1)
                    {
                        Console.WriteLine("Matching boxes are:");
                        Console.WriteLine(new string(row));
                        Console.WriteLine(new string(next));
                        return;
                    }
                }
            }
        }

        private static void Part1()
        {
            IEnumerable<Dictionary<char, int>> rows = File.ReadAllText("./input.txt")
                .Split(Environment.NewLine)
                .Select(s => s.ToCharArray()
                    .GroupBy(c => c)
                    .ToDictionary(c => c.Key, c => c.Count()));

            int twoCount = 0;
            int threeCount = 0;

            foreach (Dictionary<char, int> values in rows)
            {
                if (values.Values.Any(v => v == 2))
                {
                    twoCount++;
                }

                if (values.Values.Any(v => v == 3))
                {
                    threeCount++;
                }
            }

            Console.WriteLine($"two count: {twoCount}");
            Console.WriteLine($"three count: {threeCount}");
            Console.WriteLine($"checksum: {twoCount * threeCount}");
        }
    }
}
