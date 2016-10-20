import java.io.*;
import java.util.*;
import java.math.*;
import static java.math.BigInteger.*;

class Main {
    public static void main(String[] args) {
	Scanner sc=new Scanner(System.in);
	int n=sc.nextInt();
	int k=sc.nextInt();
	BigInteger[][] dp=new BigInteger[n+1][k+1];
	dp[0][0]=ONE;
	for(int i=0;i<=n;++i){
	    if(i>=1)dp[i][0]=ZERO;
	    for(int j=1;j<=k;++j){
	        dp[i][j]=dp[i][j-1];
		if(j<=i)dp[i][j]=dp[i][j].add(dp[i-j][j]);
	    }
	}
	System.out.println(dp[n][k]);
    }
}
