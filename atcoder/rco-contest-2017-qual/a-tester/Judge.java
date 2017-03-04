import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Scanner;

public class Judge {

    static class Result {
        TestCase input;
        long score;

        @Override
        public String toString() {
            StringBuilder builder = new StringBuilder();
            builder.append("raw score:" + score + "\n");
            builder.append("score:" + ((score + 9999) / 10000) + "\n");
            return builder.toString();
        }
    }

    static class Piece {
        int[] x, y;

        Piece(int[] x, int[] y) {
            this.x = x;
            this.y = y;
        }

        long calcScore(TestCase testcase) {
            long score = 1;
            for (int i = 0; i < this.x.length; i++) {
                score *= testcase.field[this.y[i] - 1][this.x[i] - 1];
            }
            return score;
        }

        boolean isConnected() {
            final int size = this.x.length;
            // ワーシャルフロイド法を使って連結性を判定
            boolean[][] reachable = new boolean[size][size];
            for (int i = 0; i < size; ++i) {
                for (int j = 0; j < i; ++j) {
                    boolean adjacent = Math.abs(x[i] - x[j]) + Math.abs(y[i] - y[j]) == 1;
                    reachable[i][j] = reachable[j][i] = adjacent;
                }
            }
            for (int i = 0; i < size; ++i) {
                for (int j = 0; j < size; ++j) {
                    for (int k = 0; k < size; ++k) {
                        reachable[j][k] |= reachable[j][i] && reachable[i][k];
                    }
                }
            }
            for (int i = 0; i < size; ++i) {
                for (int j = 0; j < i; ++j) {
                    if (!reachable[i][j]) {
                        return false;
                    }
                }
            }
            return true;
        }
    }


    /**
     * 解答の出力をScannerから読み込んでバリデーションを行う
     * バリデーションに失敗した場合はRuntimeExceptionをスローする
     */
    static Piece[] readOutput(TestCase testcase, Scanner sc) throws Exception {
        final int count = sc.nextInt();
        boolean[][] used = new boolean[testcase.H][testcase.W];
        Piece[] ret = new Piece[count];
        for (int i = 0; i < count; ++i) {
            int[] x = new int[testcase.K];
            int[] y = new int[testcase.K];
            for (int j = 0; j < testcase.K; ++j) {
                y[j] = sc.nextInt();
                x[j] = sc.nextInt();
                if (x[j] <= 0 || testcase.W < x[j] || y[j] <= 0 || testcase.H < y[j]) {
                    throw new RuntimeException("position (" + x[j] + "," + y[j] + ") is out of range.");
                }
                if (used[y[j] - 1][x[j] - 1]) {
                    throw new RuntimeException("position (" + x[j] + "," + y[j] + ") used more than once.");
                }
                used[y[j] - 1][x[j] - 1] = true;
            }
            ret[i] = new Piece(x, y);
            if (!ret[i].isConnected()) {
                throw new RuntimeException("piece " + i + " is not connected.");
            }
        }
        if (sc.hasNext()) {
            throw new RuntimeException("there are too many coordinates.");
        }
        return ret;
    }

    /**
     * 盤面とピースの情報を元にスコアを計算する
     * 渡すデータの内容は正しい形式であることが前提
     */
    static Result calcScore(TestCase testcase, Piece[] output) {
        Result res = new Result();
        res.input = testcase;
        for (Piece piece : output) {
            res.score += piece.calcScore(testcase);
        }
        return res;
    }

    public static void main(String[] args) throws Exception {
        if (args.length < 2) {
            System.err.println("usage: java Judge input_file_path output_file_path");
            System.exit(1);
        }
        Path inputFile = Paths.get(args[0]);
        Path outputFile = Paths.get(args[1]);
        try (Scanner input = new Scanner(inputFile); Scanner output = new Scanner(outputFile)) {
            TestCase testcase = new TestCase(input);
            Piece[] pieces = readOutput(testcase, output);
            Result res = calcScore(testcase, pieces);
            System.out.print(res);
        }
    }

}
