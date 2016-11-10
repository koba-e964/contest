#include <queue>
#include <cstdio>
#include <vector>
#define REP(i,s,n)for(int i=s;i<n;i++)
const int N=10000;
int t[N],d[N],n,x,y,v,w;
std::vector<int>e[N];
int f(int x,int y){return x+y?(x%10!=y%10)+f(x/10,y/10):0;}
int main(){
  REP(i,2,100)REP(j,2,9999/i+1)t[i*j]=1;
  REP(i,1000,N)REP(j,1000,N)if(!t[i]&&!t[j]&&f(i,j)<2)e[i].push_back(j);
  for(scanf("%d",&n);n--;printf("%d\n",d[y])){
    scanf("%d%d",&x,&y);
    REP(j,0,N)d[j]=1e9;
    std::queue<int>q;
    for(q.push(x);!q.empty();){
      int p=q.front();q.pop();
      v=p%N;
      if(d[v]>p/N)REP(j,0,e[v].size())d[v]=p/N,q.push(e[v][j]+p-v+N);
    }
  }
}
