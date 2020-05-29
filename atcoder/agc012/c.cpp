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

pair<VI, VI> dfs(ll n, int v) {
  if (n == 1) {
    return make_pair(vector<int>(), vector<int>());
  }
  if (n % 2 == 1) {
    auto sub = dfs(n - 1, v + 1);
    sub.first.push_back(v);
    sub.second.insert(sub.second.begin(), v);
    return sub;
  }
  auto sub = dfs(n / 2, v + 1);
  sub.first.push_back(v);
  sub.second.push_back(v);
  return sub;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll n;
  cin >> n;
  auto s = dfs(n + 1, 1);
  VI t(s.first);
  for (int v: s.second) t.push_back(v);
  cout << t.size() << endl;
  REP(i, 0, t.size()) {
    cout << t[i] << (i + 1 == (int)t.size() ? "\n" : " ");
  }
}
