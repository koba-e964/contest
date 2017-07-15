#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 123456;

VI edges[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  REP(i, 0, n - 1) {
    int a, b;
    cin >> a >> b;
    a--, b--;
    edges[a].push_back(b);
    edges[b].push_back(a);
  }
  queue<PI> que;
  que.push(PI(0, 1));
  que.push(PI(n - 1, 2));
  VI col(n, 0);
  while (not que.empty()) {
    PI t = que.front(); que.pop();
    int v = t.first;
    int c = t.second;
    if (col[v] != 0) { continue; }
    col[v] = c;
    REP(i, 0, edges[v].size()) {
      int w = edges[v][i];
      que.push(PI(w, c));
    }
  }
  int diff = 0;
  REP(i, 0, n) {
    if (col[i] == 1) {
      diff += 1;
    } else {
      diff -= 1;
    }
  }
  cout << (diff > 0 ? "Fennec" : "Snuke") << endl;
}
