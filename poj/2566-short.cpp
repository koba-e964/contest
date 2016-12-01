#include <algorithm>
#include <cstdio>
#define R(n)for(int i=0;i<(int)(n);i++)
typedef long long l;
typedef std::pair<l,int>P;
l a[100010],t,w,j;
P p[100010];
l m(l x){return x>0?x:-x;}
int n,k,c,b,s;
int main(){
  for(;scanf("%d%d",&n,&k)&&n+k;){
    R(n)scanf("%lld",a+i);
    p[n]=P(w,0);
    R(n)w+=a[i],p[i]=P(w,i+1);
    std::sort(p,p+n+1);
    R(k){
      scanf("%lld",&t);
      j=9e9;c=b=s=w=0;
      R(n){
	if(s==i)w=p[++s].first-p[i].first;
	for(;s<=n;s++,w+=p[s].first-p[s-1].first){
	  if(m(t-w)<m(t-j))j=w,c=p[i].second,b=p[s].second;
	  if(s>n-1||w>t)break;
	}
	w-=p[i+1].first-p[i].first;
      }
      if(b<c)s=c,c=b,b=s;
      printf("%lld %d %d\n",j,c+1,b);
    }
  }
}
