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
typedef pair<int, int> PL;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  vector<vector<PL> > pool(n + 1);
  REP(i, 0, m) {
    ll t;
    int l, r;
    cin >> t >> l >> r;
    l--;
    pool[l].push_back(PL(t, 1));
    pool[r].push_back(PL(t, 2));
  }
  set<ll> events;
  VL last(n);
  REP(i, 0, n) {
    for (PL k: pool[i]) {
      ll t = k.first;
      if (k.second == 1) {
        events.insert(t);
      } else {
        events.erase(t);
      }
    }
    if (events.size() == 0) {
      last[i] = 0;
    } else {
      last[i] = *events.rbegin();
    }
  }
  ll tot = 0;
  REP(i, 0, n) {
    tot += last[i];
  }
  cout << tot << endl;
}
