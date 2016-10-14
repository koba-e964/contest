#include<algorithm>
#include<cstdio>
#define R(n)for(i=0;i<n;i++)
int i,n,x,y,m,c;long long e[1<<17],v[1<<17],a[1<<17];int main(){scanf("%d",&n);R(n)scanf("%d%d",&x,&y),e[2*i]=x*2LL*n+n+i,e[i-~i]=i-~y*2LL*n;x=2*n;std::sort(e,e+x);R(x)m=std::max(m,c+=e[i]%x/n*2-1);printf("%d\n",m);R(m)v[i]=i;R(x)c=e[i]%x,y=c%n,c/n?a[y]=v[--m]:v[m++]=a[y];R(n)printf("%lld\n",a[i]+1);}
