using System;

namespace Day7
{
    public class Worker
    {
        public Step Step { get; private set; }
        public int TimeRemaining { get; private set; }
        public bool IsAvailable => Step == null;

        public (bool, char?) StepTime()
        {
            if(--TimeRemaining == 0)
            {
                Step.IsCompleted = true;
                char c = Step.StepId;
                Step = null;
                return (true, c);
            }

            return (false, null);
        }

        public void StartStep(Step s, int time)
        {
            if(!IsAvailable)
            {
                throw new InvalidOperationException("This worker is already working");
            }

            Step = s;
            TimeRemaining = time;
            Step.IsStarted = true;
        }
    }
}
