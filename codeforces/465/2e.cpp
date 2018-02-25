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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 11000;
const int W = 105;
const int inf = 1e8;
int node_size;
PI node[N]; // (d, -1): digit d, (x, y): [x] ? [y]
int numop[N];
int dp[N][W], dp2[N][W]; // max, min

void chmax(int &x,int y) {
  x=max(x,y);
}
void chmin(int &x,int y) {
  x = min(x, y);
}


PI parse(const string &s, int pos) {
  int n = s.length();
  assert (pos < n);
  if (s[pos] == '(') {
    PI sub = parse(s, pos + 1);
    int len = sub.first;
    assert (s[pos + 1 + len] == '?');
    PI sub2 = parse(s, pos + 1 + len + 1);
    int len2 = sub2.first;
    assert (s[pos + 1 + len + 1 + len2] == ')');
    node[node_size++] = PI(sub.second, sub2.second);
    return PI(1 + len + 1 + len2 + 1, node_size - 1);
  }
  int dig = s[pos] - '0';
  node[node_size++] = PI(dig, -1);
  return PI(1, node_size - 1);
}

bool plus_count;

void dfs(int v) {
  if (node[v].second == -1) {
    dp[v][0] = dp2[v][0] = node[v].first;
    numop[v] = 0;
    return;
  }
  int left = node[v].first;
  int right = node[v].second;
  dfs(left);
  dfs(right);
  numop[v] = numop[left] + numop[right] + 1;
  REP(i, 0, W) {
    dp[v][i] = -inf;
    dp2[v][i] = inf;
  }
  REP(i, 0, min(numop[left], W - 1) + 1) {
    REP(j, 0, min(numop[right], W - 1) + 1) {
      // TODO currently, counting + (not -)
      int targetidx = i + j + (plus_count ? 1 : 0);
      if (targetidx < W) {
	chmax(dp[v][targetidx], dp[left][i] + dp[right][j]);
	chmin(dp2[v][targetidx], dp2[left][i] + dp2[right][j]);
      }
      targetidx = i + j + (plus_count ? 0 : 1);
      if (targetidx < W) {
	chmax(dp[v][targetidx], dp[left][i] - dp2[right][j]);
	chmin(dp2[v][targetidx], dp2[left][i] - dp[right][j]);
      }
    }
  }
  if (DEBUG) {
    cerr << "dp[" << v << "]:";
    REP(i, 0, min(numop[v], W - 1) + 1) {
      cerr << " "<< dp[v][i];
    }
    cerr << endl;
    cerr << "dp2[" << v << "]:";
    REP(i, 0, min(numop[v], W - 1) + 1) {
      cerr << " "<< dp2[v][i];
    }
    cerr << endl;
  }
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string e;
  int p, m;
  cin >> e >> p >> m;
  plus_count = p < m;
  PI res = parse(e, 0);
  int root = res.second;
  if (DEBUG) {
    REP(i, 0, node_size) {
      cerr << "node[" << i << "]=" << node[i].first << " " << node[i].second
	   <<endl;
    }
  }
  dfs(root);
  cout << dp[root][plus_count ? p : m] << "\n";
}
