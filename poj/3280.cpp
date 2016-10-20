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

const int N=2200;
int n,m,x,y;
char s[N],c;
string r;
int u[26],v[26];
ll dp[N][N];
ll inf=1e16;
int main(void){
  scanf("%d%d",&n,&m);
  scanf("%2200s",s);
  r=s;
  reverse(r.begin(),r.end());
  REP(i,0,n){
    scanf("%*[\n]%c%d%d",&c,&x,&y);
    u[c-97]=min(x,y);
  }
  REP(i,0,N){
    REP(j,0,N)dp[i][j]=inf;
  }
  dp[0][0]=0;
  REP(i,0,m+1){
    REP(j,0,m+1){
      if(i==0&&j==0)continue;
      ll mi=inf;
      if(i>0){
	mi=min(mi,dp[i-1][j]+u[s[i-1]-'a']);
      }
      if(j>0){
	mi=min(mi,dp[i][j-1]+u[r[j-1]-'a']);
      }
      if(i>0&&j>0&&s[i-1]==r[j-1]){
	mi=min(mi,dp[i-1][j-1]);
      }
      dp[i][j]=mi;
    }
  }
  ll mi=inf;
  REP(i,0,m){
    mi=min(mi,dp[i][m-1-i]);
  }
  REP(i,0,m+1){
    mi=min(mi,dp[i][m-i]);
  }
  printf("%lld\n",mi);
}
