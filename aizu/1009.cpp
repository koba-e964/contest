#include<stdio.h>
int main(){
  int x,y;
  for(;scanf("%d%d",&x,&y)>0;){
    for(;y!=0;){
      int r=x%y;x=y;y=r;
    }
    printf("%d\n",x);
  }
}
