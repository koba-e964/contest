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
  int n;
  cin >> n;
  vector<string> s(n);
  map<string, int> tbl;
  REP(i, 0, n) cin >> s[i];
  set<string> u;
  u.insert("}");
  REP(i, 0, n) {
    const string &p = *u.lower_bound(s[i]);
    tbl[s[i]]++;
    if (p == s[i]) {
      cout << s[i] << " " << tbl[s[i]] << "\n";
    } else {
      int len = -1;
      REP(j, 1, s[i].size() + 1) {
        string que = s[i].substr(0, j);
        const string &q = *u.lower_bound(que);
        if (q.substr(0, j) == que) continue;
        len = j;
        break;
      }
      if (len == -1) len = s[i].size();
      cout << s[i].substr(0, len) << "\n";
      u.insert(s[i]);
    }
  }
}
