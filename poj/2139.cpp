#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N =301;
int dist[N][N];
const int inf = 0x1fffffff;
int main(void){
  int n, m;
  scanf("%d%d",&n,&m);
  REP(i,0,n){
    REP(j,0,n){
      dist[i][j] = inf;
    }
  }
  REP(i,0,m){
    int p;
    scanf("%d",&p);
    VI t(p);
    REP(j,0,p){
      int v;
      scanf("%d",&v);
      t[j]=v-1;
    }
    REP(j,0,p){
      REP(k,0,p){
	dist[t[j]][t[k]]=1;
      }
    }
  }
  REP(i,0,n){
    dist[i][i] =0;
  }
  REP(k,0,n){
    REP(i,0,n){
      REP(j,0,n){
	dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  int mi = inf;
  REP(i,0,n){
    int tot=0;
    REP(j,0,n){
      tot+=dist[i][j];
    }
    mi=min(tot,mi);
  }
  printf("%d\n",mi * 100 / (n - 1));
}
