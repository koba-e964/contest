#include <cmath>
#include <cstdio>
#define R(i,n) for(int i=0;i<n;i++)
#define N 1002
int x[N],y[N],a[N],s[N],e[N][N],o[N],n,d,p,q,c;
int r(int x){return x-s[x]?s[x]=r(s[x]):x;}
int main(){scanf("%d%d",&n,&d);R(i,n)scanf("%d%d",x+i,y+i);R(i,n)R(j,n)if(i-j&&pow(x[s[i]=i]-x[j],2)+pow(y[i]-y[j],2)<=d*d)e[i][o[i]++]=j;for(;scanf("%*[\n]%c%d",&c,&p)>1;){if(c>80)scanf("%d",&q),puts(r(--p)-r(--q)?"FAIL":"SUCCESS");else{a[--p]=1;R(j,o[p])if(a[q=e[p][j]])s[r(p)]=r(q);}}}
