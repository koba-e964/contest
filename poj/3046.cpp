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
ll mod = 1000000;

const int W=100100;
int f[1010];
ll dp[2][W];
int t,a,s,b,x;
int main(void){
  scanf("%d%d%d%d",&t,&a,&s,&b);
  REP(i,0,a){
    scanf("%d",&x);
    f[x-1]++;
  }
  // dp[i][j] = \sum_{k=0}^{f[i]} dp[i-1][j-k]
  REP(j,0,W)dp[0][j]=1;
  REP(i,0,t){
    int u=i%2;
    REP(j,0,a+1){
      dp[1-u][j]=dp[u][j]-(j>=f[i]+1?dp[u][j-f[i]-1]%mod:0)+mod;
      dp[1-u][j]%=mod;
    }
    REP(j,1,a+1){
      dp[1-u][j]+=dp[1-u][j-1];
      dp[1-u][j]%=mod;
    }
  }
  cout << (dp[t%2][b]-dp[t%2][s-1]+mod)%mod << endl;
}
