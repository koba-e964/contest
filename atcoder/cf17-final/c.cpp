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
const int DEBUG = 0;

bool ok2(const VI &cptbl, int x) {
  if (DEBUG) {
    cerr <<"ok2";
    for (auto y:cptbl)cerr << " " << y;
    cerr << " x = " << x << endl;
  }
  REP(i, 0, 24) {
    if (not cptbl[i]) {
      continue;
    }
    for (int j = i + 24 - x + 1; j <= i + 24 + x - 1; ++j) {
      if (j % 24 != i && cptbl[j % 24]) {
	return false;
      }
    }
  }
  return true;
}

bool ok(const VI &freq, int x) {
  if (DEBUG) {
    cerr << "ok";
    for (auto f: freq) cerr << " " << f;
    cerr << " x = " << x << endl;
  }
  VI tbl(24);
  int one = 0;
  REP(i, 0, 13) {
    int f = freq[i];
    if (f >= 3) {
      return false;
    }
    if ((i == 0 || i == 12) && f == 2) {
      return false;
    }
    if ((i != 0 && i != 12)  && f == 2) {
      tbl[i] = tbl[24 - i] = 1;
    } else if (f == 1) {
      one |= 1 << i;
    }
  }
  if (DEBUG) {
    cerr << "one = " << one << endl;
  }
  REP(bits, 0, 1 << 13) {
    if ((bits & one) != bits) { continue; }
    VI cptbl(tbl);
    REP(i, 0, 13) {
      if ((one & 1 << i) == 0) { continue; }
      if (bits & 1 << i) {
	cptbl[i] = 1;
      } else {
	cptbl[(24 - i) % 24] = 1;
      }
    }
    if(DEBUG)
      cerr << "bits = " << bits << endl;
    if (ok2(cptbl, x)) {
      return true;
    }
  }
  return false;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI d(n + 1, 0);
  VI freq(13);
  freq[0] = 1;
  REP(i, 0, n) {
    cin >> d[i + 1];
    freq[d[i + 1]] += 1;
  }
  sort(d.begin(), d.end());
  int lo = 0;
  int hi = 24 / (n + 1) + 1;
  while (hi - lo > 1) {
    int mid = (hi + lo) / 2;
    if (ok(freq, mid)) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  cout << lo << "\n";
}
