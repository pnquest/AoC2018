using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Day5
{
    class Program
    {
        static void Main(string[] args)
        {
            List<char> chars = File.ReadAllText("./input.txt").ToCharArray().ToList();
            var copy = CopyList(chars);
            Part1(copy);
            copy = CopyList(chars);
            Part2(copy);
            Console.ReadKey(true);
        }

        private static List<char> CopyList(List<char> chars)
        {
            char[] copy = new char[chars.Count];
            chars.CopyTo(copy);
            return chars.ToList();
        }



        private static void Part1(List<char> chars)
        {
            ReactPolymer(chars);

            Console.WriteLine($"Remaining count is {chars.Count}");
        }

        private static void Part2(List<char> chars)
        {
            int shortest = int.MaxValue;
            for(char c = 'a'; c <= 'z'; c++)
            {
                char upper = char.ToUpper(c);
                List<char> copy = CopyList(chars);
                copy.RemoveAll(i => i == c || i == upper);

                ReactPolymer(copy);

                if(copy.Count < shortest)
                {
                    shortest = copy.Count;
                }
            }

            Console.WriteLine($"Shortest part 2 is: {shortest}");
        }

        private static void ReactPolymer(List<char> chars)
        {
            int size = chars.Count;
            Stack<int> indexesToRemove = new Stack<int>();
            do
            {
                size = chars.Count;
                for (int i = 0; i < chars.Count - 1; i++)
                {
                    if (char.ToUpper(chars[i]) == char.ToUpper(chars[i + 1]) && chars[i] != chars[i + 1])
                    {
                        indexesToRemove.Push(i++);
                        indexesToRemove.Push(i);
                    }
                }

                while (indexesToRemove.Any())
                {
                    chars.RemoveAt(indexesToRemove.Pop());
                }
            } while (chars.Count < size);
        }
    }
}
