#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

mt19937 mt;

void output(int k, const vector<PI> &e) {
  string sp(2, ' ');
  cout << sp << "{" << endl;
  cout << sp << "  vector<PI> e;" << endl;
  for (auto p: e) {
    cout << sp << "  e.push_back(PI(" << p.first + 1 << ", " << p.second + 1
         << "));" << endl;
  }
  cout << sp << "  tbl[" << k << "] = e;" << endl;
  cout << sp << "}" << endl;
}

vector<PI> gen_random_graph(int n, double p) {
  vector<PI> ans;
  REP(i, 0, n) {
    REP(j, 0, i) {
      double gen = (double) mt() / (double) mt.max();
      if (gen <= p) {
        ans.push_back(PI(j, i));
      }
    }
  }
  return ans;
}

int find_k(int n, const vector<PI> &e) {
  VI g(n);
  for (auto p: e) {
    int x = p.first, y = p.second;
    g[x] |= 1 << y;
    g[y] |= 1 << x;
  }
  // O(2^n n^3)
  int ans = 0;
  REP(i, 0, n) {
    vector<vector<bool>> dp(n, vector<bool>(1 << n));
    dp[i][1 << i] = true;
    REP(bits, 0, 1 << n) {
      REP(j, 0, n) {
        if (!(bits & 1 << j)) continue;
        int pre = bits ^ 1 << j;
        REP(k, 0, n) {
          if (!(g[j] & 1 << k)) continue;
          if (!(bits & 1 << k) || j == k) continue;
          if (dp[k][pre]) {
            dp[j][bits] = true;
            break;
          }
        }
      }
    }
    REP(j, i + 1, n) {
      if (dp[j][(1 << n) - 1]) ans++;
    }
  }
  return ans;
}

int main(void) {
  vector<PI> t;
  t.push_back(PI(0, 1));
  output(2, t);
  cout << "test = " << find_k(2, t) << endl;
  int n = 12;
  set<int> seen;
  REP(test_cnt, 0, 10000) {
    vector<PI> g = gen_random_graph(n, 0.3);
    int k = find_k(n, g);
    if (k > 60) {
      cerr << "k = " << k << endl;
      continue;
    }
    if (seen.count(k)) continue;
    output(k, g);
    seen.insert(k);
  }
}
