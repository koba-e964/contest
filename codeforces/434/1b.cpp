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
#include <unordered_map>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  int mayoke = mt19937(0xe869120)() % 20;
  vector<string> s(n);
  unordered_map<string, set<int> > tbl;
  REP(i, 0, n) {
    cin >> s[i];
    REP(j, 0, 9) {
      s[i][j] += mayoke;
    }
    REP(j, 0, 9) {
      REP(k, 1, 10 - j) {
	string t = s[i].substr(j, k);
	if (tbl[t].size() <= 1) {
	  tbl[t].insert(i);
	}
      }
    }
  }
  vector<string> ans(n);
  VI len(n, 100);
  for (auto &p: tbl) {
    if (p.second.size() != 1) {
      continue;
    }
    string u = p.first;
    int idx = *p.second.begin();
    if (len[idx] > (int) u.length()) {
      len[idx] = u.length();
      ans[idx] = u;
    }
  }
  REP(i, 0, n) {
    REP(j, 0, ans[i].size()) {
      ans[i][j] -= mayoke;
    }
    cout << ans[i] << "\n";
  }
}
