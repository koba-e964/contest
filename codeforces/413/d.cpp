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
typedef pair<ll, ll> PL;

const ll inf = 1e16;
const int B = 34;

ll calc(ll a, ll b, const VL &x) {
  int n = x.size();
  n = min(n, B); // Truncate unnecessary elements
  set<PL> que;
  que.insert(PL(1, 1));
  REP(i, 0, n + 1) {
    set<PL> qnew;
    for (set<PL>::iterator it = que.begin(); it != que.end(); ++it) {
      PL t = *it;
      if (t.first >= a && t.second >= b) {
	return i;
      }
      if (i < n) {
	ll nx = t.first * x[i];
	ll ny = t.second * x[i];
	if (t.first < a) {
	  qnew.insert(PL(nx, t.second));
	}
	if (t.second < b) {
	  qnew.insert(PL(t.first, ny));
	}
      }
    }
    que = qnew;
  }
  return inf;
}

int main(void){
  cin.tie(0);
  ios::sync_with_stdio(false);
  ll a, b, h, w;
  int n;
  cin >> a >> b >> h >> w >> n;
  VL x(n);
  REP(i, 0, n) {
    cin >> x[i];
  }
  sort(x.rbegin(), x.rend());
  ll mi = inf;
  mi = min(mi, calc((a + h - 1) / h, (b + w - 1) / w, x));
  mi = min(mi, calc((a + w - 1) / w, (b + h - 1) / h, x));
  cout << (mi == inf ? -1 : mi) << endl;
}
