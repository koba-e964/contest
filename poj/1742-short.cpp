#include <algorithm>
#include <cstdio>
#define R(i,n)for(i=0;i<n;i++)
int d[1<<17],a[2110],b[2110],z[1<<17],t,n,m,v,p,i,j;
int main(){
  for(;scanf("%d%d",&n,&m)&&n;printf("%d\n",n)){
    p=0;
    R(i,n*2)scanf("%d",a+i);
    R(i,n){
      for(v=1;v<=a[i+n];v*=2)b[p++]=a[i]*v,a[i+n]-=v;b[p++]=a[i]*a[i+n];
    }
    std::sort(b,b+p);
    R(j,m)d[j+1]=0,z[j]=m-j;
    d[0]=1;n=m;
    R(i,p){
      t=0;
      R(j,n){
        v=z[j];
	if(v<b[i])break;
        if(d[v-b[i]])d[v]=1;
	else z[t++]=v;
      }
      n=t;
    }
    n=0;R(i,m)n+=d[i+1];
  }
}
