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

const int N = 100100;
set<int> g[N];
set<int> rem;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m;
  cin >> n >> m;
  REP(i, 0, m) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    g[a].insert(b);
    g[b].insert(a);
  }
  REP(i, 0, n) rem.insert(i);
  queue<int> two;
  REP(i, 0, n) {
    if (g[i].size() == 2) {
      two.push(i);
    }
  }
  while (not two.empty()) {
    int v = two.front(); two.pop();
    if (rem.count(v) == 0 || g[v].size() != 2) continue;
    //DEBUGP(v);
    auto it = g[v].begin();
    int a = *(it++);
    int b = *(it++);
    /*
    DEBUGP(a);
    DEBUGP(b);
    */
    rem.erase(v);
    g[v].clear();
    g[a].erase(v);
    g[b].erase(v);
    g[a].insert(b);
    g[b].insert(a);
    if (g[a].size() == 2) two.push(a);
    if (g[b].size() == 2) two.push(b);
  }
  cout << (rem.size() == 2 ? "Yes" : "No") << endl;
}
