#include <algorithm>
#include <cstdio>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef pair<ll,int> PLI;

const int N = 2014;
ll d[N][N];
vector<pair<ll,int> > tr[N];

void t(int v) {
  puts(v ? "YES" : "NO");
  exit(0);
}

void dfs(int r,int w,int par,ll dist){
  if(d[r][w]!=dist)t(0);
  for(PLI e:tr[w]){
    int x=e.second;
    if(par!=x)dfs(r,x,w,dist+e.first);
  }
}

int n;
int u[N]={};
pair<ll,int> dist[N];
const ll inf=1e15;
int main(void){
  scanf("%d",&n);
  REP(i,0,n)REP(j,0,n) {
    scanf("%lld",&d[i][j]);
  }
  // find an MST, Prim
  u[0] = 1;
  REP(i,0,n)dist[i]=make_pair(d[0][i],0);
  REP(_,1,n){
    pair<PLI,int> mi(PLI(inf,-1),-1);
    REP(i,0,n){
      if(u[i])continue;
      mi=min(mi,make_pair(dist[i],i));
    }
    int w=mi.first.second;
    int v=mi.second;
    ll c=mi.first.first;
    if(c==0)t(0);
    u[v]=1;
    tr[w].push_back(PLI(c,v));
    tr[v].push_back(PLI(c,w));
    REP(i,0,n)dist[i]=min(dist[i],PLI(d[v][i],v));
  }
  // distance for all pairs
  REP(i,0,n)dfs(i,i,-1,0);
  t(1);
}
