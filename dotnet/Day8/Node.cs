using System.Collections.Generic;
using System.Linq;

namespace Day8
{
    public class Node
    {
        public List<Node> Children { get; } = new List<Node>();
        public List<int> Metadata { get; } = new List<int>();
        public int Id { get; set; }
        int NumberOfNodes { get; set; }
        int NumberOfMetaDatas { get; set; }

        public int MaxId => Children.Select(m => m.MaxId).Concat(new[] { this.Id }).Max();

        public int MetadataSum()
        {
            int childSum = Children.Sum(c => c.MetadataSum());

            return childSum + Metadata.Sum();
        }

        public int Value()
        {
            if(!Children.Any())
            {
                return Metadata.Sum();
            }

            int sum = 0;

            foreach(int m in Metadata)
            {
                if(m == 0)
                {
                    continue;
                }

                if(m <= Children.Count)
                {
                    sum += Children[m - 1].Value();
                }
            }

            return sum;
        }

        public static Node FromData(Queue<int> data, int id)
        {
            int numberNodes = data.Dequeue();
            int numberMetadata = data.Dequeue();

            Node n = new Node();
            n.NumberOfNodes = numberNodes;
            n.NumberOfMetaDatas = numberMetadata;
            n.Id = id;

            int nextId = n.Id + 1;
            for(int i = 0; i < numberNodes; i++)
            {
                Node s = FromData(data, nextId);
                nextId = s.MaxId + 1;
                n.Children.Add(s);
            }

            for(int i = 0; i < numberMetadata; i++)
            {
                n.Metadata.Add(data.Dequeue());
            }

            return n;
        }


    }
}
