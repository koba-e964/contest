#include <algorithm>
#include <cstdio>
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
int a[21];
int dp[2][1<<20];
int main(){
  int n,m,p,k;
  scanf("%d%d",&n,&m);
  REP(i,0,n) {
    scanf("%d",&p);
    REP(j,0,p){
      scanf("%d",&k);
      a[i]|=1<<k-1;
    }
  }
  dp[0][0]=1;
  REP(i,0,n){
    int t=i%2;
    REP(bits,0,1<<m)dp[1-t][bits]=0;
    REP(b,0,m){
      if(a[i]&1<<b){
	REP(bits,0,1<<m){
	  if(!(bits&(1<<b))){
	    dp[1-t][bits^1<<b]+=dp[t][bits];
	  }
	}
      }
    }
  }
  int sum=0;
  REP(bits,0,1<<m)
    sum+=dp[n%2][bits];
  printf("%d\n",sum);
}
