import java.math.*;
import java.io.*;

class Main
{
	static final int MOD=1777777777;

static int mo(int k)
{
	long cur=1;
	long sum=0;
	for(int i=k;i>=2;i--)
	{
		sum+=(i%2==1)?-cur:(cur);
		sum%=MOD;
		cur*=i;
		cur%=MOD;
	}
	return (int)(sum%MOD);
}

static long comb(long n,int k)
{
	n%=MOD;
	BigInteger prod=BigInteger.ONE;
	for(int i=0;i<k;i++)
	{
		prod=prod.multiply(BigInteger.valueOf(n-i));
		prod=prod.divide(BigInteger.valueOf(i+1));
	}
	return prod.mod(BigInteger.valueOf(MOD)).longValue();

}


static int solve(long n,int k)
{
	long cm=comb(n,k);
	return (int)((cm*mo(k))%MOD);
}




public static void main(String[] args)throws IOException
{
	long n;
	int k;
	String line=new BufferedReader(new InputStreamReader(System.in)).readLine();
	String[] split=line.split("\\s");
	n=Long.parseLong(split[0]);
	k=Integer.parseInt(split[1]);
	System.out.printf("%d\n",(int)solve(n,k));
}
}