import java.io.*;
import java.util.*;
import java.math.*;
import static java.math.BigInteger.*;

class Solution {
    public static void main(String[] args) {
        FastScanner sc = new FastScanner();
        out = new PrintWriter(new BufferedOutputStream(System.out));

        int t = sc.nextInt();
        for (int case_nr = 1; case_nr <= t; ++case_nr) {
            BigInteger n = new BigInteger(sc.next());
            int l = sc.nextInt();
            BigInteger[] c = new BigInteger[l];
            for (int i = 0;i < l; ++i) {
                c[i] = new BigInteger(sc.next());
            }
            // Find c[i] = p * q
            BigInteger p = null, q = null;
            for (int i = 1; i < l; ++i) {
                BigInteger g = c[0].gcd(c[i]);
                if (!g.equals(ONE) && !g.equals(c[0])) {
                    p = g;
                    q = c[0].divide(p);
                    break;
                }
            }
            BigInteger[][] poss = new BigInteger[2][l + 1];
            BigInteger[] ans = null;
            poss[0][0] = p;
            poss[1][0] = q;
            for (int i = 0; i < 2; ++i) {
                boolean ok = true;
                for (int j = 0; j < l; ++j) {
                    BigInteger[] qr = c[j].divideAndRemainder(poss[i][j]);
                    if (qr[1].equals(ZERO)) {
                        poss[i][j + 1] = qr[0]; 
                    } else {
                        ok = false;
                        break;
                    }
                }
                if (ok) {
                    ans = poss[i];
                    break;
                }
            }
            SortedSet<BigInteger> srt = new TreeSet<BigInteger>();
            for (BigInteger b: ans) srt.add(b);
            BigInteger[] inv = srt.toArray(new BigInteger[0]);
            out.print("Case #" + case_nr + ": ");
            for (int i = 0; i < l + 1; ++i) {
                int idx = -1;
                for (int j = 0; j < 26; ++j) {
                    if (inv[j].equals(ans[i])) {
                        idx = j;
                        break;
                    }
                }
                out.print((char)('A' + idx));
            }
            out.println();
        }
        out.close();
    }
    public static PrintWriter out;
}

// https://qiita.com/p_shiki37/items/a0f6aac33bf60f5f65e4
class FastScanner {
    private final InputStream in = System.in;
    private final byte[] buffer = new byte[1024];
    private int ptr = 0;
    private int buflen = 0;
    private boolean hasNextByte() {
        if (ptr < buflen) {
            return true;
        }else{
            ptr = 0;
            try {
                buflen = in.read(buffer);
            } catch (IOException e) {
                e.printStackTrace();
            }
            if (buflen <= 0) {
                return false;
            }
        }
        return true;
    }
    private int readByte() { if (hasNextByte()) return buffer[ptr++]; else return -1;}
    private static boolean isPrintableChar(int c) { return 33 <= c && c <= 126;}
    public boolean hasNext() { while(hasNextByte() && !isPrintableChar(buffer[ptr])) ptr++; return hasNextByte();}
    public String next() {
        if (!hasNext()) throw new NoSuchElementException();
        StringBuilder sb = new StringBuilder();
        int b = readByte();
        while(isPrintableChar(b)) {
            sb.appendCodePoint(b);
            b = readByte();
        }
        return sb.toString();
    }
    public long nextLong() {
        if (!hasNext()) throw new NoSuchElementException();
        long n = 0;
        boolean minus = false;
        int b = readByte();
        if (b == '-') {
            minus = true;
            b = readByte();
        }
        if (b < '0' || '9' < b) {
            throw new NumberFormatException();
        }
        while(true){
            if ('0' <= b && b <= '9') {
                n *= 10;
                n += b - '0';
            }else if(b == -1 || !isPrintableChar(b)){
                return minus ? -n : n;
            }else{
                throw new NumberFormatException();
            }
            b = readByte();
        }
    }
    public int nextInt() {
        long nl = nextLong();
        if (nl < Integer.MIN_VALUE || nl > Integer.MAX_VALUE) throw new NumberFormatException();
        return (int) nl;
    }
    public double nextDouble() { return Double.parseDouble(next());}
}
