import java.util.*;
import java.math.*;
import static java.math.BigInteger.*;

class Main {
    public static BigInteger fib(int l) {
	BigInteger x = ONE; // sum
	BigInteger y = ZERO;
	BigInteger z = ZERO; // cur
	BigInteger w = ONE;
	int e = l + 1;
	while (e > 0) {
	    if (e % 2 == 1) {
		BigInteger tmp = y.multiply(w);
		BigInteger newx = x.multiply(z);
		BigInteger newy = x.add(y).multiply(z.add(w)).subtract(newx);
		newx = newx.add(tmp);
		x = newx;
		y = newy;
	    }
	    BigInteger tmp = w.multiply(w);
	    BigInteger newz = z.multiply(z).add(tmp);
	    BigInteger neww = z.shiftLeft(1).multiply(w).add(tmp);
	    z = newz;
	    w = neww;
	    e /= 2;
	}
	return x;
    }
    public static void main(String[] args) {
	Scanner sc = new Scanner(System.in);
	int l = sc.nextInt();
	if (l == 2) {
	    System.out.println("3\nINF");
	    return;
	}
	System.out.println(l);
	if (l % 2 == 1) {
	    System.out.println(fib(l));
	} else {
	    BigInteger tmp = fib(l / 2);
	    System.out.println(fib(l).subtract(tmp.multiply(tmp)));
	}
    }
}
