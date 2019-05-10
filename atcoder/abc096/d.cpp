#include <stdio.h>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

const int N=60000;
int pr[N];

int main(void){
  REP(i,2,N)pr[i]=1;
  REP(i,2,N){
    if(pr[i])for(int j=2*i;j<N;j+=i)pr[j]=0;
  }
  int n;
  scanf("%d",&n);
  int pos=2;
  while(n>0){
    while(pos%10!=1||pr[pos]==0)pos++;
    printf("%d%c",pos,n==1?'\n':' ');
    pos++;
    n--;
  }
}
