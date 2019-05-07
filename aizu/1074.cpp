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
  while (cin >> n && n) {
    map<int, vector<string> > tbl;
    map<string, int> score;
    REP(i, 0, n) {
      string name;
      int len; cin >> name >> len;
      REP(j, 0, len) {
        int t; cin >> t;
        tbl[t].push_back(name);
      }
      score[name] = 0;
    }
    for (auto e: tbl) {
      auto &v = e.second;
      for (auto f: v) {
        score[f] += n - v.size() + 1;
      }
    }
    pair<int, string> mi(1e9, "");
    for (auto e: score) mi = min(mi, make_pair(e.second, e.first));
    cout << mi.first << " " << mi.second << endl;
  }
}
