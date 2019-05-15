#include <stdio.h>
#include <vector>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

int n,m,k[200100],d[200100],t[200100];

int last[200100];
int sched[400100];

bool check(int x){
  REP(i,0,200100)last[i]=-1;
  REP(i,0,m)if(d[i]<=x)last[t[i]]=max(last[t[i]],d[i]);
  int ksum=0;
  REP(i,0,n)ksum+=k[i];
  REP(i,0,400100)sched[i]=0;
  REP(i,1,n+1)if(last[i]>=0)sched[last[i]]+=k[i-1];
  int nec=0;
  int has=0;
  REP(i,1,x+1){
    has++;
    int y=min(sched[i],has);
    has-=y;
    nec+=y;
    ksum-=y;
  }
  nec+=2*ksum;
  return nec<=x;
}

int main(void){
  scanf("%d%d",&n,&m);
  REP(i,0,n)scanf("%d",k+i);
  REP(i,0,m)scanf("%d%d",d+i,t+i);
  int ps=400010,fl=0;
  while(ps-fl>1){
    int mid=(ps+fl)/2;
    if(check(mid))ps=mid;
    else fl=mid;
  }
  printf("%d\n",ps);
}
