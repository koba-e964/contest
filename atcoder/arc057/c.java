import java.math.*;
import java.io.*;
import java.util.*;

class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        String s = sc.next();
        BigInteger bi = new BigInteger(s);
        BigInteger bione = bi.add(BigInteger.ONE);
        BigInteger bisq = bi.multiply(bi);
        BigInteger hyaku = BigInteger.valueOf(100);
        BigInteger bionesq = bione.multiply(bione).subtract(BigInteger.ONE);
        while (true) {
            BigInteger a1 = bisq.divide(hyaku);
            if (bisq.remainder(hyaku).compareTo(BigInteger.ZERO) > 0) {
                a1 = a1.add(BigInteger.ONE);
            }
            BigInteger a2 = bionesq.divide(hyaku);
            if (a1.compareTo(a2) <= 0) {
            bisq = a1;
            bionesq = a2;
            } else {
            break;
            }
        }
        System.out.println(bisq);
        return;
    }
}
