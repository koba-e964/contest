#include <iostream>
#include <algorithm>
#define TEST 0

typedef long long ll;
using namespace std;
const int N=1000001;
const ll MOD=1000000007;
int k,n;
ll dp[N];
ll dpe[N];
int main(void){
	cin>>n>>k;
	fill_n(dp,N,0);
	fill_n(dpe,N,0);
	dp[0]=1;
	dpe[0]=1;
	for(int i=1;i<=n;i++){
		dpe[i]=dp[i-1];
		if(i>=k){
			dpe[i]+=MOD-(i>=k+1?dp[i-k-1]:1);
			dpe[i]%=MOD;
		}
		dp[i]=dp[i-1]+dpe[i];
		dp[i]%=MOD;
	}
	if(TEST){
		for(int i=0;i<=n;i++){
			cout<<i<<": ("<<dp[i]<<","<<dpe[i]<<")"<<endl;
		}
	}
	cout<<((dpe[n]-dpe[n-1]+MOD)%MOD)<<endl;
}
