import java.util.Random;

public class Generator {

    public static void main(String[] args) throws Exception {
        long seed = new Random().nextInt();
        for (int i = 0; i < args.length; ++i) {
            if (args[i].equals("-seed")) {
                seed = Long.parseLong(args[++i]);
            }
        }
        TestCase testcase = new TestCase(seed);
        System.out.println(testcase);
    }

}
