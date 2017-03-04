import java.util.Random;

public class Generator {
	public static void main(String[] args) {
		long seed = new Random().nextInt();
		for (int i = 0; i < args.length; ++i) {
			if (args[i].equals("-seed")) {
				seed = Long.parseLong(args[++i]);
			}
		}
		System.err.println("seed:" + seed);
		System.out.println(new TestCase(seed).toString());
	}
}
