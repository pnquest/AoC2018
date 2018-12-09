using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Day7
{
    class Program
    {
        static void Main(string[] args)
        {
            Part1();
            Part2();
            Console.ReadKey(true);
        }

        private static Dictionary<char, Step> LoadSteps()
        {
            Dictionary<char, Step> steps = new Dictionary<char, Step>();
            foreach (string l in File.ReadLines("./input.txt"))
            {
                var slc = l.AsSpan();
                char parent = slc.Slice(5, 1)[0];
                char child = slc.Slice(36, 1)[0];

                if (!steps.ContainsKey(parent))
                {
                    steps[parent] = new Step(parent);
                }

                if (!steps.ContainsKey(child))
                {
                    steps[child] = new Step(child);
                }

                Step parentStep = steps[parent];
                Step childStep = steps[child];

                parentStep.Children.Add(childStep);
                childStep.Parents.Add(parentStep);
            }

            return steps;
        }

        private static void Part2()
        {
            Dictionary<char, Step> steps = LoadSteps();
            List<char> completedOrder = new List<char>();

            WorkerPool pool = new WorkerPool(5);

            int time = 0;

            while(steps.Values.Any(s => !s.IsCompleted))
            {
                if(pool.FreeWorkers > 0)
                {
                    pool.QueueWork(steps.Values
                        .Where(s => !s.IsCompleted && !s.IsStarted && (!s.Parents.Any() || s.Parents.All(p => p.IsCompleted)))
                        .OrderBy(s => s.StepId)
                        .Take(pool.FreeWorkers)
                        .ToArray());
                }

                completedOrder.AddRange(pool.StepWorkers());
                time++;
            }

            Console.WriteLine($"Part 2: {time}");
        }

        private static void Part1()
        {
            Dictionary<char, Step> steps = LoadSteps();

            List<char> completedOrder = new List<char>();

            while (steps.Values.Any(s => !s.IsCompleted))
            {
                Step start = steps.Values.Where(s => !s.IsCompleted && (!s.Parents.Any() || s.Parents.All(p => p.IsCompleted))).OrderBy(s => s.StepId).First();
                start.IsCompleted = true;
                completedOrder.Add(start.StepId);
            }

            Console.WriteLine($"part 1: {new String(completedOrder.ToArray())}");
        }
    }
}
