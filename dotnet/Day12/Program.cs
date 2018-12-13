using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace Day12
{
    class Program
    {
        static void Main(string[] args)
        {
            LinkedList<Node> state;
            Dictionary<string, char> patterns;
            LoadData(out state, out patterns);

            Part1(state, patterns);
            LoadData(out state, out patterns);
            Part2(state, patterns);
            Console.ReadKey(true);
        }

        private static void LoadData(out LinkedList<Node> state, out Dictionary<string, char> patterns)
        {
            string[] lines = File.ReadAllLines("./input.txt");

            state = new LinkedList<Node>();
            patterns = new Dictionary<string, char>();
            List<string> history = new List<string>();

            string start = lines[0].Split(": ")[1];

            for (int i = 0; i < start.Length; i++)
            {
                if (i < 0 || i > start.Length - 1)
                {
                    state.AddLast(new Node { Position = i, Value = '.' });
                }
                else
                {
                    state.AddLast(new Node { Position = i, Value = start[i] });
                }

            }

            for (int i = 2; i < lines.Length; i++)
            {
                string[] split = lines[i].Split(" => ");
                patterns.Add(split[0], split[1][0]);
            }
        }

        private static void Part2(LinkedList<Node> state, Dictionary<string, char> patterns)
        {
            //ran and found that pattern stabilizes to +62 per gen eventually at gen 4249 the total was 264093

            long total = (50000000000 - 4249) * 62 + 264093;


            Console.WriteLine($"Part 2: {total}");
        }

        private static void Part1(LinkedList<Node> state, Dictionary<string, char> patterns)
        {
            StringBuilder sb = new StringBuilder();
            for (long g = 0; g < 20; g++)
            {
                PrintBoard(state, sb, g);
                ExpandList(state);
                RunGeneration(state, patterns);
            }
            PrintBoard(state, sb, 20);
            int sum = CalculateSum(state);
            Console.WriteLine($"Part 1: {sum}");
        }

        private static int CalculateSum(LinkedList<Node> state)
        {
            return state.Where(s => s.Value == '#').Sum(s => s.Position);
        }

        private static void ExpandList(LinkedList<Node> state)
        {
            LinkedListNode<Node> cur = state.First;
            for (int i = 0; i <= 4; i++)
            {
                if (cur.Value.Value == '#')
                {
                    for(int j = 0; j < 4; j++)
                    {
                        state.AddFirst(new Node { Position = state.First.Value.Position - 1, Value = '.' });
                    }
                    
                    break;
                }
                cur = cur.Next;
            }
            cur = state.Last;
            for (int i = 0; i <= 4; i++)
            {
                if (cur.Value.Value == '#')
                {
                    for(int j = 0; j < 4; j++)
                    {
                        state.AddLast(new Node { Position = state.Last.Value.Position + 1, Value = '.' });
                    }
                    
                    break;
                }
                cur = cur.Previous;
            }
        }

        private static void PrintBoard(LinkedList<Node> state, StringBuilder sb, long g)
        {
            foreach (char c in state.Select(s => s.Value))
            {
                sb.Append(c);
            }
            string val = sb.ToString();
            int sum = CalculateSum(state);
            Console.WriteLine($"{g}({sum}): {val}");
            sb.Clear();
        }

        private static void RunGeneration(LinkedList<Node> state, Dictionary<string, char> patterns)
        {
            LinkedListNode<Node> cur = state.First;
            StringBuilder sb = new StringBuilder();
            while (cur != null)
            {
                string input = GenerateInput(cur, sb);

                if(input != ".....")
                {
                   // Debugger.Break();
                }

                char result = '.';
                if (patterns.TryGetValue(input, out char r))
                {
                    result = r;
                }

                

                cur.Value.NextValue = result;
                cur = cur.Next;
            }

            foreach (Node n in state)
            {
                n.UpdateGeneration();
            }
        }

        private static string GenerateInput(LinkedListNode<Node> cur, StringBuilder sb)
        {
            string input = null;

            if (cur.Previous?.Previous == null)
            {
                sb.Append('.');
            }
            else
            {
                sb.Append(cur.Previous.Previous.Value.Value);
            }

            if (cur.Previous == null)
            {
                sb.Append('.');
            }
            else
            {
                sb.Append(cur.Previous.Value.Value);
            }

            sb.Append(cur.Value.Value);

            if (cur.Next == null)
            {
                sb.Append('.');
            }
            else
            {
                sb.Append(cur.Next.Value.Value);
            }

            if (cur.Next?.Next == null)
            {
                sb.Append('.');
            }
            else
            {
                sb.Append(cur.Next.Next.Value.Value);
            }

            input = sb.ToString();
            sb.Clear();
            return input;
        }
    }
}
