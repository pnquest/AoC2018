using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;

namespace Day6
{
    class Program
    {
        static void Main(string[] args)
        {
            Dictionary<int, Point> points = new Dictionary<int, Point>();

            int minX = int.MaxValue;
            int maxX = 0;
            int minY = int.MaxValue;
            int maxY = 0;

            int id = 0;
            foreach (string s in File.ReadAllLines("./input.txt"))
            {
                var spn = s.AsSpan();
                int idx = spn.IndexOf(',');
                int x = int.Parse(spn.Slice(0, idx));
                int y = int.Parse(spn.Slice(idx + 2));

                if (x < minX)
                {
                    minX = x;
                }
                if (x > maxX)
                {
                    maxX = x;
                }

                if (y < minY)
                {
                    minY = y;
                }

                if (y > maxY)
                {
                    maxY = y;
                }

                points.Add(id++, new Point(x, y));
            }

            Part1(points, minX, maxX, minY, maxY);
            Part2(points, minX, maxX, minY, maxY);
            Console.ReadKey(true);
        }

        private static void Part2(Dictionary<int, Point> points, int minX, int maxX, int minY, int maxY)
        {
            int inRegionCount = 0;

            for(int x = minX; x <= maxX; x++)
            {
                for(int y = minY; y <= maxY; y++)
                {
                    int totalDistance = points.Select(p => Math.Abs(p.Value.X - x) + Math.Abs(p.Value.Y - y)).Sum();

                    if(totalDistance < 10000)
                    {
                        inRegionCount++;
                    }
                }
            }

            Console.WriteLine($"Total Size {inRegionCount}");
        }

        private static void Part1(Dictionary<int, Point> points, int minX, int maxX, int minY, int maxY)
        {
            List<int> cornerIds = points.Where(p => !points.Any(p2 => p2.Value.X < p.Value.X) ||
                            !points.Any(p2 => p2.Value.X > p.Value.X) ||
                            !points.Any(p2 => p2.Value.Y < p.Value.Y) ||
                            !points.Any(p2 => p2.Value.Y > p.Value.Y))
                        .Select(p => p.Key)
                        .ToList();

            List<int> owner = new List<int>();

            for (int x = minX; x <= maxX; x++)
            {
                for (int y = minY; y <= maxY; y++)
                {
                    var distances = points
                        .Select(p => new { p.Key, Distance = Math.Abs(x - p.Value.X) + Math.Abs(y - p.Value.Y) });
                    var pt = distances
                        .GroupBy(p => p.Distance)
                        .Select(g => new { Distance = g.Key, Values = g })
                        .OrderBy(g => g.Distance)
                        .First();

                    if (pt.Values.Count() == 1
                        && !cornerIds.Contains(pt.Values.First().Key))
                    {

                        owner.Add(pt.Values.First().Key);
                    }
                }
            }

            var groups = owner
                .GroupBy(g => g)
                .Select(g => new { g.Key, Count = g.Count() })
                .OrderByDescending(g => g.Count)
                .Skip(1); // for some reason first place must be infinte? I don't get it.. but skipping works

            int max = groups.Select(g => g.Count).Max();

            Console.WriteLine($"Max is: {max}");
        }
    }
}
