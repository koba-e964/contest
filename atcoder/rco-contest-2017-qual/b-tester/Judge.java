import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Scanner;

public class Judge {
	static class Result {
		long originalScore;
		long truncatedScore;

		@Override
		public String toString() {
			StringBuilder builder = new StringBuilder();
			builder.append("raw score: ").append(originalScore).append("\n");
			builder.append("score: ").append(truncatedScore).append("\n");
			return builder.toString();
		}
	}

	private Result execute(TestCase input, String output) {
		Result res = new Result();
		System.out.println(input.mapToString());
		try (Scanner sc = new Scanner(output)) {
			char[] route = sc.next().toCharArray();
			if(route.length != input.K){
				throw new RuntimeException(String.format("length of route does not equal to K(%d).", input.K));
			}
			for (int j = 0; j < input.K; j++) {
				char c = route[j];
				if (TestCase.DIRS.indexOf(c) == -1){
					throw new RuntimeException("S[" + j + "]=" + c + " is not valid.");
				}
			}
			
			res.originalScore = input.simulate(route);
			res.truncatedScore = input.toTruncatedScore(res.originalScore);
			System.out.println(input.trailMapToString(route));
		}
		return res;
	}
	
	public static String loadText(String path)
	{
		try {
			return new String(Files.readAllBytes(Paths.get(path)), StandardCharsets.UTF_8);
		} catch (IOException e) {
			System.err.println("error occurred when reading text from file: " + path);
			e.printStackTrace();
		}
		return null;
	}
	

	public static void main(String[] args) throws IOException {
		// by seed
		if (args.length >= 3 && args[0].equals("-seed")) {
			long seed = Long.parseLong(args[1]);
			String outputPath = args[2];
			Result res = new Judge().execute(new TestCase(seed), loadText(outputPath));
			System.out.println("seed: " + seed);
			System.out.println(res);
			return;
		}
		// by input file
		if (args.length >= 2) {
			String inputPath = args[0];
			String outputPath = args[1];
			Result res = new Judge().execute(new TestCase(loadText(inputPath)), loadText(outputPath));
			System.out.println(res);
			return;
		}
		System.err.print(
				"Usage: java Judge -seed (seed other than 0 e.g. 123) (output_file_path)\n" +
				"       java Judge (input_file_path) (output_file_path)\n"
						);
		System.exit(1);
	}

}
