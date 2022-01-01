using System;
using System.Runtime.InteropServices;

public class Program
{
    [DllImport("dynamic_lib.dll")]
    extern static int sum(int x, int y);

    public static void Main()
    {
        try
        {
            Console.WriteLine("Starting");
            var z = sum(1, 2);
            Console.WriteLine(z);
            Console.ReadLine();
        }
        finally
        {
            Console.WriteLine("Exiting");
        }
    }
}