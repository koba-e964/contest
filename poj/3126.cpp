#include <queue>
#include <cstdio>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 10000;
int dp[N];
VI edges[N];

int diff(int x,int y){
  int cnt=0;
  while(x||y){
    cnt += x%10 != y%10 ? 1: 0;
    x/=10;y/=10;
  }
  return cnt;
}

int main(void){
  int n;
  scanf("%d",&n);
  REP(i,2,N)dp[i]=1;
  REP(i,2,100){
    REP(j,2,(N+i-1)/i){
      dp[i*j]=0;
    }
  }
  REP(i,1000,N){
    if(not dp[i])continue;
    REP(j,i+1,N){
      if(not dp[j])continue;
      if(diff(i,j)>1)continue;
      edges[i].push_back(j);
      edges[j].push_back(i);
    }
  }
  VI dist(N);
  REP(i, 0, n) {
    int x,y;
    scanf("%d%d",&x,&y);
    REP(j,0,N)dist[j]=1e9;
    queue<PI> que;
    que.push(PI(x,0));
    while(not que.empty()){
      PI p=que.front(); que.pop();
      int v=p.first;
      if(dist[v]<=p.second){continue;}
      dist[v]=p.second;
      REP(j,0,edges[v].size()){
	que.push(PI(edges[v][j],p.second+1));
      }
    }
    printf("%d\n",dist[y]);
  }
}
