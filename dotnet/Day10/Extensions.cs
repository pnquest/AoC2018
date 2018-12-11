using System;
using System.Collections.Generic;
using System.Linq;

namespace Day10
{
    public static class Extensions
    {
        public static double CalulateVariance(this IEnumerable<double> values)
        {
            double[] vals = values.ToArray();

            int count = vals.Length;
            double sum = vals.Sum();
            double average = sum / count;

            double numerator = vals.Sum(v => Math.Pow(v - average, 2));

            return numerator / (count - 1);
        }

        public static double CalulateVariance(this IEnumerable<int> values)
        {
            int[] vals = values.ToArray();

            int count = vals.Length;
            double sum = (double)vals.Sum();
            double average = sum / count;

            double numerator = vals.Sum(v => Math.Pow(v - average, 2));

            return numerator / (count - 1);
        }
    }
}
