using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;
using System.Diagnostics;

namespace Day1
{
    class Program
    {
        static void Main(string[] args)
        {
            Stopwatch stp = new Stopwatch();
            stp.Start();
            long[] values = File.ReadAllText("./input.txt").Split("\n").Select(s => long.Parse(s)).ToArray();
            Part1(values);
            Part2(values);
            stp.Stop();
            Console.WriteLine(stp.ElapsedMilliseconds);
        }

        private static void Part1(long[] values)
        {
            long sum = values.Sum();
            Console.WriteLine($"The result is {sum}");
        }

        private static void Part2(long[] values)
        {
            HashSet<long> visited = new HashSet<long>();
            visited.Add(0);

            long current = 0;
            while(true)
            {
                foreach(long v in values)
                {
                    current += v;
                    if(!visited.Add(current))
                    {
                        Console.WriteLine($"The first value visted twice is {current}");
                        return;
                    }
                }
            }
            throw new Exception("Well, this didn't work like I planned...");
        }
    }
}
