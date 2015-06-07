#include <algorithm>
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
typedef pair<int, int> PI;
const double EPS=1e-9;

const int N = 10010;
VI factors[N];
int dp[N];

int mex(const set<int> &s) {
  int v = 0;
  while(1) {
    if (s.count(v) == 0) {
      return v;
    }
    v++;
  }
}

int main(void){
  int n;
  cin >> n;
  REP (i, 2, N) {
    if (factors[i].size()) {
      continue;
    }
    for (int j = i; j < N; j += i) {
      factors[j].push_back(i);
    }
  }
  dp[1] = 0;
  REP(i, 2, N) {
    set<int> sub;
    REP(j, 0, factors[i].size()) {
      int f = factors[i][j];
      sub.insert(dp[i / f]);
      if (i % (f * f) == 0) {
	sub.insert(dp[i / f / f]);
      }
    }
    dp[i] = mex(sub);
  }
  int xo = 0;
  REP(i, 0, n) {
    int m;
    cin >> m;
    xo ^= dp[m];
  }
  cout << (xo ? "Alice" : "Bob") << endl;
}
