using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

class Solution
{
    static void Main(string[] args)
    {
        var input = Console.ReadLine();

        Console.WriteLine(
            int.TryParse(input, out var number)
                ? number.ToString()
                : "Bad String");
    }
}