#include<cstdio>
#define R(i,s,n)for(int i=s;i<n;i++)
int a[21],d[2][1<<20],n,m,p,k,t;
int main(){
  scanf("%d%d",&n,&m);
  R(i,0,n)for(scanf("%d",&p);p--;a[i]|=1<<k-1)scanf("%d",&k);
  d[0][0]=1;
  k=1<<m;
  R(i,0,n){t=1-i%2;
    R(s,0,k)d[t][s]=0;
    R(b,0,m)
      R(s,0,k)a[i]&1<<b&&s&1<<b?d[t][s]+=d[1-t][s^1<<b]:0;
  }
  R(s,p=0,k)p+=d[n%2][s];
  printf("%d\n",p);
}
