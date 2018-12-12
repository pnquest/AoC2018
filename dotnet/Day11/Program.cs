using System;
using System.Drawing;

namespace Day11
{
    class Program
    {
        static void Main(string[] args)
        {
            const long input = 8772;

            long[,] values = new long[300, 300];

            for (int ix = 0; ix < 300; ix++)
            {
                for (int iy = 0; iy < 300; iy++)
                {
                    int x = ix + 1;
                    int y = iy + 1;

                    long rackId = (x + 10);
                    long power = rackId * y;
                    power += input;
                    power *= rackId;

                    if (power < 100)
                    {
                        power = 0;
                    }
                    else
                    {
                        power = power / 100 % 10 - 5;
                    }

                    values[ix, iy] = power;
                }
            }

            Part1(values);
            Part2SummedAreaTable(values);
            //Part2(values);
            Console.ReadKey(true);
        }

        private static void Part2(long[,] values)
        {
            Point? maxPiont = null;
            long? maxValue = null;
            int? maxSize = null;

            for(int s = 1; s<= 300; s++)
            {
                for (int x = 0; x < 300 - (s - 1); x++)
                {
                    for (int y = 0; y < 300 - (s - 1); y++)
                    {
                        long sum = 0;

                        for (int cx = x; cx < x + s; cx++)
                        {
                            for (int cy = y; cy < y + s; cy++)
                            {
                                sum += values[cx, cy];
                            }
                        }

                        if (!maxValue.HasValue || maxValue.Value < sum)
                        {
                            maxValue = sum;
                            maxPiont = new Point(x + 1, y + 1);
                            maxSize = s;
                        }
                    }
                }
            }
            

            Console.WriteLine($"Part 2: {maxPiont.Value.X},{maxPiont.Value.Y},{maxSize.Value} {maxValue.Value}");
        }

        private static void Part2SummedAreaTable(long[,] values)
        {
            long[,] sat = new long[300, 300];

            for(int x = 0; x < 300; x++)
            {
                for(int y = 0; y < 300; y++)
                {
                    long node = values[x, y];

                    if(y > 0)
                    {
                        node += sat[x, y - 1];
                    }

                    if(x > 0)
                    {
                        node += sat[x - 1, y];
                    }

                    if(x > 0 && y > 0)
                    {
                        node -= sat[x - 1, y - 1];
                    }

                    sat[x, y] = node;
                }
            }

            Point? maxPiont = null;
            long? maxValue = null;
            int? maxSize = null;

            for (int s = 1; s < 300; s++)
            {
                for (int x = 0; x < 300 - (s - 1); x++)
                {
                    for (int y = 0; y < 300 - (s - 1); y++)
                    {
                        Point d = new Point(x + s - 1, y + s - 1);
                        Point a = new Point(d.X - s, d.Y - s);
                        Point b = new Point(d.X, d.Y - s);
                        Point c = new Point(d.X - s, d.Y);


                        long iD = sat[d.X, d.Y];

                        long iA = 0;
                        if (a.X >= 0 && a.Y >= 0)
                        {
                            iA = sat[a.X, a.Y];
                        }

                        long iB = 0; 
                        if(b.X >= 0 && b.Y >= 0)
                        {
                            iB = sat[b.X, b.Y];
                        }
                        long iC = 0;
                        if (c.X >= 0 && c.Y >= 0)
                        {
                            iC = sat[c.X, c.Y];
                        }

                        long sum = iD + iA - iB - iC;

                        if (!maxValue.HasValue || maxValue.Value < sum)
                        {
                            maxValue = sum;
                            maxPiont = new Point(x + 1, y + 1);
                            maxSize = s;
                        }
                    }
                }
            }

            if(sat[299,299] > maxValue.Value)
            {
                maxPiont = new Point(0, 0);
                maxValue = sat[299, 299];
                maxSize = 300;
            }

            Console.WriteLine($"Part 2 (SAT): {maxPiont.Value.X},{maxPiont.Value.Y},{maxSize.Value} {maxValue.Value}");
        }

        private static void Part1(long[,] values)
        {
            Point? maxPiont = null;
            long? maxValue = null;

            for (int x = 0; x < 298; x++)
            {
                for (int y = 0; y < 298; y++)
                {
                    long sum = 0;

                    for (int cx = x; cx < x + 3; cx++)
                    {
                        for (int cy = y; cy < y + 3; cy++)
                        {
                            sum += values[cx, cy];
                        }
                    }

                    if (!maxValue.HasValue || maxValue.Value < sum)
                    {
                        maxValue = sum;
                        maxPiont = new Point(x + 1, y + 1);
                    }
                }
            }

            Console.WriteLine($"Part 1: {maxPiont.Value.X},{maxPiont.Value.Y} {maxValue.Value}");
        }
    }
}
