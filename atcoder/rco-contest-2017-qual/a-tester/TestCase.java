import java.security.SecureRandom;
import java.util.Scanner;

public class TestCase {

    static final int MIN_K = 8;
    static final int MAX_K = 8;
    static final int MIN_SIZE = 50;
    static final int MAX_SIZE = 50;

    final int H, W, K;
    final int[][] field;

    TestCase(long seed) throws Exception {
        SecureRandom rnd = SecureRandom.getInstance("SHA1PRNG");
        rnd.setSeed(seed);
        this.H = rnd.nextInt(MAX_SIZE - MIN_SIZE + 1) + MIN_SIZE;
        this.W = rnd.nextInt(MAX_SIZE - MIN_SIZE + 1) + MIN_SIZE;
        this.K = rnd.nextInt(MAX_K - MIN_K + 1) + MIN_K;
        this.field = new int[H][W];
        for (int i = 0; i < H; ++i) {
            for (int j = 0; j < W; ++j) {
                // 盤面の数字をランダムに生成
                this.field[i][j] = rnd.nextInt(10);
            }
        }
    }

    TestCase(Scanner sc) throws Exception {
        this.H = sc.nextInt();
        this.W = sc.nextInt();
        this.K = sc.nextInt();
        this.field = new int[this.H][this.W];
        for (int i = 0; i < this.H; i++) {
            char[] row = sc.next().toCharArray();
            for (int j = 0; j < this.W; j++) {
                this.field[i][j] = row[j] - '0';
            }
        }
    }


    @Override
    public String toString() {
        StringBuilder builder = new StringBuilder();
        builder.append(this.H + " " + this.W + " " + this.K + "\n");
        for (int i = 0; i < this.H; ++i) {
            for (int j = 0; j < this.W; ++j) {
                builder.append(this.field[i][j]);
            }
            builder.append("\n");
        }
        return builder.toString();
    }

}
