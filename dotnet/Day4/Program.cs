using System;
using System.Collections.Generic;
using System.IO;
using System.Text.RegularExpressions;
using System.Linq;
using System.Diagnostics;

namespace Day4
{
    class Program
    {
        static readonly Regex _guardRegex = new Regex(@"\[(?<timestamp>\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] Guard #(?<id>\d+) begins shift");

        static readonly Regex _otherRegex = new Regex(@"\[(?<timestamp>\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (wakes up|falls asleep)");

        static void Main(string[] args)
        {
            Stopwatch sw = new Stopwatch();
            sw.Start();
            string[] lines = File.ReadAllLines("./input.txt");
            Dictionary<int, Guard> guards = new Dictionary<int, Guard>();
            Guard currentGuard = null;
            int lastMinute = 0;
            Entry.EventType lastEvent = Entry.EventType.StartShift;

            Entry[] entries = lines.Select(ParseLine).OrderBy(e => e.TimeStamp).ToArray();

            foreach (Entry entry in entries)
            {
                ParseEntries(guards, ref currentGuard, ref lastMinute, ref lastEvent, entry);
            }

            Part1(guards);
            Part2(guards);
            sw.Stop();
            Console.WriteLine($"Runtime: {sw.ElapsedMilliseconds}");
            Console.ReadKey(true);
        }

        private static void Part2(Dictionary<int, Guard> guards)
        {
            var sleepyGuard = guards
                            .Select(g => new { Guard = g.Value, Asleep = g.Value.SeelpingMinutes.Values.SelectMany(m => m).GroupBy(m => m).ToDictionary(m => m.Key, m => m.Count()) })
                            .Select(g => new { g.Guard, g.Asleep, MaxKey = g.Asleep.OrderByDescending(a => a.Value).FirstOrDefault().Key})
                            .OrderByDescending(g => {
                                if(g.MaxKey == 0)
                                {
                                    return 0;
                                }
                                return g.Asleep[g.MaxKey];
                            })
                            .First();

            Console.WriteLine($"The guard is {sleepyGuard.Guard.Id}");
            Console.WriteLine($"The minute is {sleepyGuard.MaxKey}");
            Console.WriteLine($"The answer is {sleepyGuard.Guard.Id * sleepyGuard.MaxKey}");
        }

        private static void Part1(Dictionary<int, Guard> guards)
        {
            Guard sleepyGuard = guards
                            .Select(g => new { Guard = g.Value, Asleep = g.Value.SeelpingMinutes.Values.SelectMany(m => m).Count() })
                            .OrderByDescending(g => g.Asleep)
                            .First().Guard;

            Console.WriteLine($"The sleepy guard is {sleepyGuard.Id}");

            int sleepyMinute = sleepyGuard.SeelpingMinutes.Values
                .SelectMany(m => m)
                .GroupBy(g => g)
                .Select(g => new { Minute = g.Key, Count = g.Count() })
                .OrderByDescending(g => g.Count)
                .First().Minute;

            Console.WriteLine($"The sleepty minute is {sleepyMinute}");

            Console.WriteLine($"The answer is {sleepyGuard.Id * sleepyMinute}");
        }

        private static void ParseEntries(Dictionary<int, Guard> guards, ref Guard currentGuard, ref int lastMinute, ref Entry.EventType lastEvent, Entry entry)
        {
            switch (entry.Event)
            {
                case Entry.EventType.StartShift:
                    if (currentGuard != null && lastEvent == Entry.EventType.FallsAsleep)
                    {
                        DateTime dt = entry.TimeStamp.Date;
                        for (int i = lastMinute; i < entry.TimeStamp.Minute; i++)
                        {
                            currentGuard.SeelpingMinutes[dt].Add(i);
                        }
                    }
                    if (!guards.ContainsKey(entry.Id.Value))
                    {
                        guards[entry.Id.Value] = new Guard { Id = entry.Id.Value };
                    }
                    currentGuard = guards[entry.Id.Value];

                    DateTime entryDate = entry.TimeStamp.Date;
                    lastMinute = entry.TimeStamp.Minute;
                    if (entry.TimeStamp.Hour != 0)
                    {
                        entryDate = entryDate.AddDays(1);
                        lastMinute = 0;
                    }
                    currentGuard.SeelpingMinutes[entryDate] = new List<int>();
                    lastEvent = Entry.EventType.StartShift;
                    break;

                case Entry.EventType.FallsAsleep:
                    lastMinute = entry.TimeStamp.Minute;
                    lastEvent = Entry.EventType.FallsAsleep;
                    break;

                case Entry.EventType.WakesUp:
                    DateTime date = entry.TimeStamp.Date;
                    for (int i = lastMinute; i < entry.TimeStamp.Minute; i++)
                    {
                        currentGuard.SeelpingMinutes[date].Add(i);
                    }
                    lastEvent = Entry.EventType.WakesUp;
                    break;
            }
        }

        private static Entry ParseLine(string line)
        {
            if(line.EndsWith("begins shift"))
            {
                Match m = _guardRegex.Match(line);
                int id = int.Parse(m.Groups["id"].Value);
                DateTime timestamp = DateTime.Parse(m.Groups["timestamp"].Value);

                return new Entry(timestamp, id, Entry.EventType.StartShift);
            }
            if(line.EndsWith("wakes up"))
            {
                Match m = _otherRegex.Match(line);
                DateTime timestamp = DateTime.Parse(m.Groups["timestamp"].Value);

                return new Entry(timestamp, null, Entry.EventType.WakesUp);
            }
            if (line.EndsWith("falls asleep"))
            {
                Match m = _otherRegex.Match(line);
                DateTime timestamp = DateTime.Parse(m.Groups["timestamp"].Value);

                return new Entry(timestamp, null, Entry.EventType.FallsAsleep);
            }

            throw new ArgumentException("bad input");
        }
    }
}
