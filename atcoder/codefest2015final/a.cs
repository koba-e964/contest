using System.Linq;
using System;

public class A {
	public static void Main(string[] args) {
		int[] val = Console.ReadLine().Split(new []{' '}).Select(x => x.Length).ToArray();
		Console.WriteLine(val[0] == 5 && val[1] == 7 && val[2] == 5 ? "valid" : "invalid");
	}
}