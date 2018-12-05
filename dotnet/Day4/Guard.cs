using System;
using System.Collections.Generic;

namespace Day4
{
    public class Guard
    {
        public int Id { get; set; }
        public Dictionary<DateTime, List<int>> SeelpingMinutes { get; } = new Dictionary<DateTime, List<int>>();
    }
}
