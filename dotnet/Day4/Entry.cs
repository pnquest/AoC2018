using System;

namespace Day4
{
    public class Entry
    {
        public DateTime TimeStamp { get; set; }
        public int? Id { get; set; }
        public EventType Event { get; set; }

        public enum EventType
        {
            StartShift,
            FallsAsleep,
            WakesUp,
        }

        public Entry(DateTime timestamp, int? id, EventType eventType)
        {
            TimeStamp = timestamp;
            Id = id;
            Event = eventType;
        }
    }
}
