import java.util.*;
import java.math.*;
import static java.math.BigInteger.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        long n = sc.nextLong();
        long h = sc.nextLong();
        BigInteger nn = BigInteger.valueOf(n);
        BigInteger hh = BigInteger.valueOf(h);
        BigInteger threshold = hh.multiply(hh.add(ONE)).divide(BigInteger.valueOf(2));
        if (nn.compareTo(threshold) >= 0) {
            long pass = 1L << 31;
            long fail = h - 1;
            while (pass - fail > 1) {
                long mid = (pass + fail) / 2;
                long tmp;
                if ((mid + h) % 2 == 1) {
                    long summit = (mid + h - 1) / 2;
                    tmp = summit * (summit + 1) - h * (h - 1) / 2;
                } else {
                    long summit = (mid + h) / 2;
                    tmp = summit * summit - h * (h - 1) / 2;
                }
                if (tmp >= n) {
                    pass = mid;
                } else {
                    fail = mid;
                }
            }
            System.out.println(pass);
        } else {
            long pass = 1L << 31;
            long fail = 0;
            while (pass - fail > 1) {
                long mid = (pass + fail) / 2;
                if (mid * (mid + 1) >= 2 * n) {
                    pass = mid;
                } else {
                    fail = mid;
                }
            }
            System.out.println(pass);
        }
    }
}
