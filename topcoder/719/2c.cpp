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

class TwoDogsOnATree {
  vector<vector<PI> > edges;
  vector<int> dp;
  void dfs(int v, int val) {
    dp[v] = val;
    REP(i, 0, edges[v].size()) {
      PI dat = edges[v][i];
      dfs(dat.first, val ^ dat.second);
    }
  }
public:
  int maximalXorSum(vector <int> parent, vector <int> w) {
    int n = w.size() + 1;
    edges = vector<vector<PI> >(n);
    dp = vector<int>(n);
    REP(i, 0, n - 1) {
      edges[parent[i]].push_back(PI(i + 1, w[i]));
    }
    dfs(0, 0);
    set<int> two;
    int ma = 0;
    REP(i, 0, n) {
      REP(j, 0, n) {
	two.insert(dp[i] ^ dp[j]);
      }
    }
    VI two_v(two.begin(), two.end());
    REP(i, 0, two_v.size()) {
      REP(j, 0, two_v.size()) {
	ma = max(ma, two_v[i] ^ two_v[j]);
      }
    }
    return ma;
  }
};
