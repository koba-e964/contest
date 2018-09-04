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

#define EX 1

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

map<string, int> memo;
int grundy(const string &s) {
  if (memo.count(s)) return memo[s];
  if (s == "") return memo[s] = 0;
  set<int> sub;
  for (char c = 'a'; c <= 'z'; ++c) {
    if (s.find(c) != string::npos) {
      string cur;
      int sum = 0;
      REP(i, 0, s.length()) {
	if (s[i] == c) {
	  sum ^= grundy(cur);
	  cur = "";
	} else {
	  cur += s[i];
	}
      }
      sum ^= grundy(cur);
      sub.insert(sum);
    }
  }
  int ans = 0;
  while (sub.count(ans)) ans++;
  return memo[s] = ans;
}

#if EX
int main(void) {
  int len;
  cin >> len;
  int lim = pow(len, len);
  map<int, vector<string> > repr;
  REP(i, 0, lim) {
    string s(len, '+');
    int v = i;
    REP(j, 0, len) {
      s[j] = 'a' + (v % len);
      v /= len;
    }
    bool ok = true;
    REP(j, 0, len - 1) if (s[j] == s[j + 1]) ok = false;
    if (not ok) continue;
    int g = grundy(s);
    if (repr[g].size() < 4) {
      repr[g].push_back(s);
    }
  }
  for (auto &k: repr) {
    int g = k.first;
    cerr << "g = " << g << ":";
    for (auto u: k.second) {
      cerr << " " << u;
    }
    cerr << endl;
  }
}
#else
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  int m;
  cin >> s >> m;
  REP(i, 0, m) {
    int l, r;
    cin >> l >> r;
    l--;
  }
}
#endif
