#include<stdio.h>

int main(){
  int k;
  for(;scanf("%d",&k)&&k;){
    int t=0;
    for(int i=0;i<k*(k-1)/2;++i){
      int v;scanf("%d",&v);t+=v;
    }
    printf("%d\n",t/(k-1));
  }
}
