using System.Drawing;

namespace Day3
{
    public class Claim
    {
        public int Id { get; set; }
        public Rectangle ClaimArea {get;set;}

        public Claim(int id, int left, int top, int width, int height)
        {
            Id = id;
            ClaimArea = new Rectangle(left, top, width, height);
        }
    }
}
