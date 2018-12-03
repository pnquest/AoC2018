using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Text.RegularExpressions;
using System.Linq;
using System.Diagnostics;

namespace Day3
{
    class Program
    {
        static void Main(string[] args)
        {
            Stopwatch sw = new Stopwatch();
            sw.Start();
            Regex lineRegex = new Regex(@"^#(?<id>\d+) @ (?<posLeft>\d+),(?<posTop>\d+): (?<width>\d+)x(?<height>\d+)$");
            string[] rows = File.ReadAllText("./input.txt").Split(Environment.NewLine);

            List<Claim> rectangles = rows.Select(s =>
            {
                Match m = lineRegex.Match(s);
                int id = int.Parse(m.Groups["id"].Value);
                int posLeft = int.Parse(m.Groups["posLeft"].Value);
                int posTop = int.Parse(m.Groups["posTop"].Value);
                int width = int.Parse(m.Groups["width"].Value);
                int height = int.Parse(m.Groups["height"].Value);

                return new Claim(id, posLeft, posTop, width, height);
            }).ToList();

            Part1(rectangles);
            Part2(rectangles);
            sw.Stop();
            Console.WriteLine($"runtime is {sw.ElapsedMilliseconds}");
            Console.ReadKey(true);
        }

        private static void Part1(List<Claim> rectangles)
        {
            long subArea = 0;

            List<Rectangle> overlaps = new List<Rectangle>();

            for (int i = 0; i < rectangles.Count; i++)
            {
                Rectangle r1 = rectangles[i].ClaimArea;
                for (int j = i; j < rectangles.Count; j++)
                {
                    if (i != j)
                    {
                        Rectangle r2 = rectangles[j].ClaimArea;

                        r2.Intersect(r1);

                        if (r2 != default(Rectangle))
                        {
                            subArea += GetSubtractableArea(r2, overlaps);
                            overlaps.Add(r2);
                        }
                    }
                }
            }

            Console.WriteLine($"The overlap area is, {subArea}");
        }

        private static void Part2(List<Claim> rectangles)
        {
            for (int i = 0; i < rectangles.Count; i++)
            {
                bool overlapped = false;
                Claim c1 = rectangles[i];
                Rectangle r1 = c1.ClaimArea;
                for (int j = 0; j < rectangles.Count; j++)
                {
                    if (i != j)
                    {
                        Rectangle r2 = rectangles[j].ClaimArea;

                        r2.Intersect(r1);

                        if (r2 != default(Rectangle))
                        {
                            overlapped = true;
                            break;
                        }
                    }
                }

                if(!overlapped)
                {
                    Console.WriteLine($"The non-overlapping claim id is {c1.Id}");
                    return;
                }
            }
        }

        private static int GetSubtractableArea(Rectangle overlap, List<Rectangle> overlaps)
        {
            List<Rectangle> subOverlap = new List<Rectangle>();
            int area = overlap.Width * overlap.Height;

            foreach(Rectangle r in overlaps)
            {
                r.Intersect(overlap);

                if(r != default(Rectangle))
                {
                    area -= GetSubtractableArea(r, subOverlap);
                    subOverlap.Add(r);
                }
            }

            if(area < 0)
            {
                throw new Exception("Something is wrong");
            }

            return area;
        }
    }
}
