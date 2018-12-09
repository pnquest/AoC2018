using System.Collections.Generic;
using System.Linq;

namespace Day7
{
    public class WorkerPool
    {
        private List<Worker> workers;
        public int FreeWorkers => workers.Count(w => w.IsAvailable);

        public WorkerPool(int numberOfWorkers)
        {
            workers = new List<Worker>(numberOfWorkers);
            for(int i = 0; i < numberOfWorkers; i++)
            {
                workers.Add(new Worker());
            }
        }

        public void QueueWork(Step[] steps)
        {
            foreach(Step step in steps)
            {
                int time = step.StepId - 'A' + 61;
                workers.First(w => w.IsAvailable).StartStep(step, time);
            }
        }

        public IEnumerable<char> StepWorkers()
        {
            foreach(Worker worker in workers.Where(w => !w.IsAvailable))
            {
                (bool isDone, char? doneChar) = worker.StepTime();

                if(isDone)
                {
                    yield return doneChar.Value;
                }
            }
        }
    }
}
