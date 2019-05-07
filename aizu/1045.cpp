#include<stdio.h>

int a[20];

int main(){
  int n;
  for(;scanf("%d",&n)&&n;){
    for(int i=0;i<n;++i)scanf("%d",a+i);
    int mi=1e9;
    for(int b=0;b<1<<n;++b){
      int d=0;
      for(int i=0;i<n;++i)
        d+=b&1<<i?a[i]:-a[i];
      if(d<0)d=-d;
      if(mi>d)mi=d;
    }
    printf("%d\n",mi);
  }
}
