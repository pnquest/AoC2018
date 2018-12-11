using System;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace Day10
{
    class Program
    {
        static void Main(string[] args)
        {
            Regex reg = new Regex(@"position=<\s*(?<px>[\-0-9]+),\s+(?<py>[\-0-9]+)> velocity=<\s*(?<vx>[\-0-9]+),\s+(?<vy>[\-0-9]+)>");

            (Point, Point)[] points = File.ReadAllLines("./input.txt").Select(l =>
            {
                Match m = reg.Match(l);

                return (new Point(int.Parse(m.Groups["px"].Value), int.Parse(m.Groups["py"].Value)), 
                    new Point(int.Parse(m.Groups["vx"].Value), int.Parse(m.Groups["vy"].Value)));
            }).ToArray();

            (Point, Point)[] nextPoints = new (Point, Point)[points.Length];

            double varianceX = points.Select(p => p.Item1.X).CalulateVariance();
            double varianceY = points.Select(p => p.Item1.Y).CalulateVariance();
            double nextVarianceX;
            double nextVarianceY;

            for(int i = 0; i < points.Length; i++)
            {
                nextPoints[i] = (new Point(points[i].Item1.X + points[i].Item2.X, points[i].Item1.Y + points[i].Item2.Y), points[i].Item2);
            }

            nextVarianceX = nextPoints.Select(p => p.Item1.X).CalulateVariance();
            nextVarianceY = nextPoints.Select(p => p.Item1.Y).CalulateVariance();

            int seconds = 0;

            while((nextVarianceX + nextVarianceY) / 2 < (varianceX + varianceY) / 2)
            {
                Array.Copy(nextPoints, points, nextPoints.Length);
                varianceX = nextVarianceX;
                varianceY = nextVarianceY;
                for (int i = 0; i < points.Length; i++)
                {
                    nextPoints[i] = (new Point(points[i].Item1.X + points[i].Item2.X, points[i].Item1.Y + points[i].Item2.Y), points[i].Item2);
                }

                nextVarianceX = nextPoints.Select(p => p.Item1.X).CalulateVariance();
                nextVarianceY = nextPoints.Select(p => p.Item1.Y).CalulateVariance();
                seconds++;
            }

            int minX = points.Min(p => p.Item1.X);
            int maxX = points.Max(p => p.Item1.X);
            int minY = points.Min(p => p.Item1.Y);
            int maxY = points.Max(p => p.Item1.Y);

            for(int y = minY - 1; y <= maxY + 1; y++)
            {
                for(int x = minX - 1; x <= maxX + 1; x++)
                {
                    if(points.Any(p => p.Item1.X == x && p.Item1.Y == y))
                    {
                        Console.ForegroundColor = ConsoleColor.DarkYellow;
                        Console.Write("#");
                        Console.ForegroundColor = ConsoleColor.Gray;
                    }
                    else
                    {
                        Console.Write(".");
                    }
                }
                Console.WriteLine();
            }

            Console.WriteLine($"Part 2: {seconds}");


            Console.ReadKey(true);
        }
    }
}
