using System;
using System.Collections.Generic;
using System.Linq;

namespace Day9
{
    class Program
    {
        static void Main(string[] args)
        {
            const long numPlayers = 463;
            const long lastValue = 71787;
            long part1 = RunGame(numPlayers, lastValue);
            Console.WriteLine($"Part 1: {part1}");
            long part2 = RunGame(numPlayers, lastValue * 100);
            Console.WriteLine($"Part 2: {part2}");
            Console.ReadKey(true);
        }

        private static long RunGame(long numPlayers, long lastValue)
        {
            LinkedListNode<long> currentNode = null;
            long currentPlayer = 0;
            LinkedList<long> circle = new LinkedList<long>();
            long[] playerScores = new long[numPlayers];
            Array.Fill(playerScores, 0);

            currentNode = circle.AddFirst(0);

            for (long i = 1; i <= lastValue; i++)
            {
                if (i % 23 == 0)
                {
                    playerScores[currentPlayer] += i;
                    LinkedListNode<long> n = currentNode;
                    for (int j = 0; j < 7; j++)
                    {
                        if (n.Next == null)
                        {
                            n = n.List.First;
                        }
                        else
                        {
                            n = n.Next;
                        }
                    }
                    if (n.Previous == null)
                    {
                        currentNode = circle.Last;
                    }
                    else
                    {
                        currentNode = n.Previous;
                    }
                    circle.Remove(n);
                    playerScores[currentPlayer] += n.Value;
                }
                else
                {
                    LinkedListNode<long> n = currentNode;
                    if (n.Previous == null)
                    {
                        n = circle.Last;
                    }
                    else
                    {
                        n = n.Previous;
                    }
                    currentNode = circle.AddBefore(n, i);
                }

                currentPlayer = (currentPlayer + 1) % numPlayers;
            }
            return playerScores.Max();
        }
    }
}
