#include <algorithm>
#include <cstdio>
#define REP(i,s,n)for(int i=(int)(s);i<(int)(n);i++)
int dp[1<<17],a[2110],b[2110],tbl[1<<17],n2,n,m,v,pos;
int main(void){
  while(scanf("%d%d",&n,&m)&&n>0){
    REP(i,pos=0,n*2)scanf("%d",a+i);
    REP(i,0,n){
      for(v=1;v<=a[i+n];v*=2)b[pos++]=a[i]*v,a[i+n]-=v;b[pos++]=a[i]*a[i+n];
    }
    std::sort(b,b+pos);
    REP(j,1,m+1)dp[j]=0,tbl[j-1]=m+1-j;
    dp[0]=1;n=m;
    REP(i,0,pos){
      n2=0;
      REP(j,0,n){
        v=tbl[j];
	if(v<b[i])break;
        if(dp[v-b[i]])
	  dp[v]=1;
	else
	  tbl[n2++]=v;
      }
      n=n2;
    }
    n=0;
    REP(i,0,m)n+=dp[i+1];
    printf("%d\n",n);
  }
}
