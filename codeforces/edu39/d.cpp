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
typedef pair<ll, int> PLI;

const int N = 510;
string s[N];

const ll inf = 1e12;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, k;
  cin >> n >> m >> k;
  REP(i, 0, n) {
    cin >> s[i];
  }
  vector<VL> skip(n, VL(k + 1, inf));
  REP(i, 0, n) {
    VI test;
    REP(j, 0, m) {
      if (s[i][j] == '1') {
	test.push_back(j);
      }
    }
    int ts = test.size();
    REP(j, 0, ts) {
      REP(l, j, ts) {
	int sk = ts - (l - j + 1);
	if (sk <= k) {
	  skip[i][sk] = min(skip[i][sk], (ll)test[l] - test[j] + 1);
	}
      }
    }
    if (ts <= k) {
      skip[i][ts] = 0;
    }
    if(0){
      cerr<<"skip[" << i << "]:";
      REP(j, 0, k + 1){
	cerr << " " <<skip[i][j];
      }
      cerr << endl;
    }
  }
  ll tot = 0;
  REP(i, 0, n) tot += skip[i][0];
  REP(i, 0, n) {
    REP(j, 1, k + 1) {
      skip[i][j] = skip[i][0] - skip[i][j];
    }
    skip[i][0] = 0;
  }
  VL dp(k + 1, -inf);
  VL dp2(k + 1, -inf);
  dp[0] = 0;
  REP(i, 0, n) {
    fill(dp2.begin(), dp2.end(), -inf);
    REP(j, 0, k + 1) {
      for (int l = k; l >= j; --l) {
	dp2[l] = max(dp2[l], dp[l - j] + skip[i][j]);
      }
    }
    swap(dp2, dp);
  }
  ll mi = -inf;
  REP(i, 0, k + 1) {
    mi = max(mi, dp[i]);
  }
  cout << tot - mi << "\n";
}
