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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 214365;
int t[N], l[N], r[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q, m;
  cin >> n >> q >> m;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(i, 0, q) {
    cin >> t[i] >> l[i] >> r[i];
    l[i]--, r[i]--;
  }
  VI idx(m);
  REP(i, 0, m) {
    cin >> idx[i];
    idx[i]--;
  }
  for (int i = q - 1; i >= 0; --i) {
    if (t[i] == 1) {
      REP(j, 0, m) {
	int v = idx[j];
	if (l[i] <= v && v <= r[i]) {
	  v = (v + r[i] - l[i] - l[i]) % (r[i] - l[i] + 1) + l[i];
	  idx[j] = v;
	}
      }
    } else {
      REP(j, 0, m) {
	int v = idx[j];
	if (l[i] <= v && v <= r[i]) {
	  v = l[i] + r[i] - v;
	  idx[j] = v;
	}
      }
    }
  }
  REP(i, 0, m) {
    cout << a[idx[i]] << (i == m - 1 ? "\n": " ");
  }
}
