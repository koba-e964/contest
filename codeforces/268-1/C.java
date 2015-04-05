import java.math.*;
import java.util.*;
import static java.math.BigInteger.*;

public class C {
	static BigInteger f(BigInteger val) {
		BigInteger s = ZERO;
		for(char c: val.toString().toCharArray()) {
			s = s.add(valueOf(c - '0'));
		}
		return s;
	}
	static BigInteger sum(BigInteger val) {
		if(val.signum() < 0) {
			return ZERO;
		}
		BigInteger TEN = valueOf(10);
		BigInteger sub = sum(val.divide(TEN).subtract(ONE)).multiply(TEN).add(val.divide(TEN).multiply(valueOf(45)));
		sub = sub.add(f(val.divide(TEN)).multiply(val.mod(TEN).add(ONE)));
		for(int i=0; i< val.mod(TEN).add(ONE).intValue(); i++) {
			sub = sub.add(valueOf(i));
		}
		return sub;
	}
	static BigInteger bin_search(BigInteger val) {
		BigInteger l = ONE;
		BigInteger r = val.add(ONE);
		while(r.subtract(l).compareTo(ONE) > 0) {
			BigInteger m = l.add(r).shiftRight(1);
			if (sum(m).compareTo(val) < 0) {
				l = m;
			} else
				r = m;
		}
		return l;
	}

	static final int SIZE = 100000;
	BigInteger[] cache;
	C() {
		Scanner scan = new Scanner(System.in);
		long n = scan.nextLong();
		BigInteger bn = valueOf(n);
		cache = new BigInteger[SIZE];
		Arrays.fill(cache, null);
		cache[0] = ZERO;
		BigInteger x = ONE;
		while(true) {
			BigInteger val = sum(x);
			if( val.mod(valueOf(n)).longValue() >= SIZE) {
				BigInteger q = val.divide(bn).multiply(bn).add(bn);
				x = bin_search(q).add(ONE);
				continue;
			}
			int r = val.mod(bn).intValue();
			if(cache[r] == null) {
				cache[r] = x;
			} else {
				System.out.println((cache[r].add(ONE)) + " " + x);
				return;
			}
			x = x.add(ONE);
		}
	}
	public static void main(String[] args) {
		new C();
	}
}