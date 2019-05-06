#include<stdio.h>
int s[101],n,k,v,o,i;int main(){for(;scanf("%d%d",&n,&k)&&n&&k;puts(o?"No":"Yes")){for(i=k;i--;scanf("%d",s+i))o=0;for(;n--;)for(i=k;i--;o|=(s[i]-=v)<0)scanf("%d",&v);}}
