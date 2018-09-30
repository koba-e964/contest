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
  int n, m;
  cin >> n >> m;
  VL s(n);
  REP(i, 0, n) {
    cin >> s[i];
    s[i] *= 2;
  }
  REP(i, 0, m) {
    int a, b;
    ll c;
    cin >> a >> b >> c;
    a--, b--;
    s[a] += c;
    s[b] += c;
  }
  sort(s.rbegin(), s.rend());
  ll dif = 0;
  REP(i, 0, n) {
    dif += s[i] * (i % 2 == 0 ? 1 : -1);
  }
  cout << (dif > 0 ? "First" : "Second") << endl;
}
