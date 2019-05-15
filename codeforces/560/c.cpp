#include <stdio.h>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int n;
char a[200010],b[200010];

int main(void){
  scanf("%d%200009s",&n,a);
  int pos=0;
  for(int i=0;i<n;i++){
    if(n-i<=1)break;
    int nxt=i+1;
    while(nxt<n&&a[i]==a[nxt])nxt++;
    if(nxt>=n)break;
    b[pos]=a[i];
    b[pos+1]=a[nxt];
    i=nxt;
    pos+=2;
  }
  b[pos]=0;
  printf("%d\n%s\n",n-pos,b);
}
