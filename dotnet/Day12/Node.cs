namespace Day12
{
    public class Node
    {
        public char Value { get; set; }
        public char? NextValue { get; set; }
        public int Position { get; set; }

        public void UpdateGeneration()
        {
            Value = NextValue.Value;
            NextValue = null;
        }

        public override string ToString()
        {
            return Position.ToString();
        }
    }
}
