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

const int N = 100100;
int dp[N];
VI child[N];

const int DEBUG = 0;

void dfs(int v) {
  VI packed;
  for (auto w: child[v]) {
    dfs(w);
    packed.push_back(dp[w]);
  }
  sort(packed.rbegin(), packed.rend());
  int mi = 0;
  REP(i, 0, child[v].size()) {
    mi = max(mi, packed[i] + i + 1);
  }
  dp[v] = mi;
  if (DEBUG) {
    cerr << "dp[" << v << "]=" << mi << endl;
  }
}

int main(void){
  int n;
  cin >> n;
  VI a(n - 1);
  REP(i, 0, n - 1) {
    cin >> a[i];
    a[i]--;
  }
  REP(i, 0, n - 1) {
    child[a[i]].push_back(i + 1);
  }
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << "child[" << i << "]:";
      for (auto j: child[i]) {
	cerr << " " << j;
      }
      cerr << endl;
    }
  }
  dfs(0);
  cout << dp[0] << endl;
}
