using System.Collections.Generic;

namespace Day7
{
    public class Step
    {
        public char StepId { get; }
        public List<Step> Parents { get; } = new List<Step>();
        public List<Step> Children { get; } = new List<Step>();
        public bool IsCompleted { get; set; } = false;
        public bool IsStarted { get; set; } = false;

        public Step(char stepId)
        {
            StepId = stepId;
        }
    }
}
