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



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  int n = s.length();
  VI t(n);
  set<char> ss;
  REP(i, 0, 26) ss.insert('a' + i);
  REP(i, 0, n) {
    int idx = distance(ss.begin(), ss.find(s[i]));
    t[i] = idx;
    ss.erase(s[i]);
  }
  if (n < 26) {
    t.push_back(0);
  } else {
    int ind = -1;
    for (int i = 25; i >= 0; --i) {
      if (t[i] == 25 - i) continue;
      ind = i;
      break;
    }
    if (ind == -1) {
      cout << -1 << endl;
      return 0;
    }
    while (t.size() > ind + 1) t.pop_back();
    t[ind]++;
  }
  if (0) {
    REP(i, 0, t.size()) cerr << " " << t[i];
    cerr << endl;
  }
  string ans(t.size(), '+');
  REP(i, 0, 26) ss.insert('a' + i);
  REP(i, 0, t.size()) {
    auto it = ss.begin();
    advance(it, t[i]);
    char c = *it;
    ans[i] = c;
    ss.erase(c);
  }
  cout << ans << endl;
}
