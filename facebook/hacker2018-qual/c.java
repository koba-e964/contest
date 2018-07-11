import java.util.*;

class Main {
    static void fail() {
        System.out.println("Impossible");
    }
    static void assertEthanFails(String a, String b) {
        int i = 0;
        int j = 0;
        while (true) {
            if (i >= a.length()) throw new RuntimeException("Ethan returns true!");
            if (j >= b.length()) return;
            if (a.charAt(i) == b.charAt(j)) {
                i += 1;
                j += 1;
                continue;
            }
            if (i == 0) {
                j += 1;
                continue;
            }
            i = 0;
        }
    }
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int t = sc.nextInt();
        for (int case_nr = 1; case_nr <= t; ++case_nr) {
            String a = sc.next();
            int n = a.length();
            int u = -1;
            for (int i = 1; i < n; ++i) {
                if (a.charAt(i) == a.charAt(0)) {
                    u = i;
                    break;
                }
            }
            System.out.print("Case #" + case_nr + ": ");
            if (u == -1) {
                fail();
                continue;
            }
            if (a.substring(0, n - u).equals(a.substring(u, n))) {
                fail();
                continue;
            }
            String b = a.substring(0, u) + a;
            System.out.println(b);
            assertEthanFails(a, b);
        }
    }
}
