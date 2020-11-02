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
const ll mod = 1e9 + 7;

vector<string> rot(const vector<string> &s) {
  int n = s.size();
  vector<string> ans(n, string(n, '+'));
  REP(i, 0, n) {
    REP(j, 0, n) {
      ans[n - j - 1][i] = s[i][j];
    }
  }
  return ans;
}

vector<string> flip(const vector<string> &s) {
  int n = s.size();
  vector<string> ans(n, string(n, '+'));
  REP(i, 0, n) {
    REP(j, 0, n) {
      ans[i][n - j - 1] = s[i][j];
    }
  }
  return ans;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<string> s(n);
  REP(i, 0, n) cin >> s[i];
  vector<string> t(n);
  REP(i, 0, n) cin >> t[i];
  REP(_, 0, 2) {
    REP(_, 0, 4) {
      if (s == t) {
        cout << "YES" << endl;
        return 0;
      }
      s = rot(s);
    }
    s = flip(s);
  }
  cout << "NO" << endl;
}
