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
  VI p(n);
  REP(i, 0, n) p[i] = i;
  set<VI> ans;
  queue<VI> que;
  que.push(p);
  while (not que.empty()) {
    VI t = que.front(); que.pop();
    if (ans.count(t))continue;
    ans.insert(t);
    REP(i, 0, n - 2) {
      if (t[i] < t[i + 1] && t[i + 1] < t[i + 2]) {
	VI u(t);
	swap(u[i], u[i + 2]);
	que.push(u);
      }
    }
  }
  int count = 0;
  for (auto k: ans) {
    cerr << count << ":";
    for (int v: k) cerr << " " << v;
    VI inv(n);
    cerr << " |";
    REP(i, 0, n) inv[k[i]] = i;
    for (int v: inv) cerr << " " << v;
    cerr << " " << (ans.count(inv) ? "Yes" : "No");
    cerr << endl;
    count++;
  }
}
