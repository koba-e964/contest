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
const ll mod = 1e9 + 7;

void output(const VI& a) {
  int n = a.size();
  REP(i, 0, n) {
    cout << a[i] << (i == n - 1 ? "\n" : " ");
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h;
  cin >> h;
  VI a(h + 1);
  REP(i, 0, h + 1) {
    cin >> a[i];
  }
  bool ok = true;
  int erri = -1;
  REP(i, 0, h) {
    if (a[i] >= 2 && a[i + 1] >= 2) {
      erri = i;
      ok = false;
      break;
    }
  }
  if (ok) {
    cout << "perfect\n";
    return 0;
  }
  int tot = 0;
  REP(i, 0, h + 1) {
    tot += a[i];
  }
  cout << "ambiguous\n";
  VI p(tot);
  int prev = 0;
  int pos = 0;
  REP(i, 0, h + 1) {
    REP(j, 0, a[i]) {
      p[pos++] = prev;
    }
    prev = pos;
  }
  output(p);
  pos = 0;
  prev = 0;
  REP(i, 0, h + 1) {
    REP(j, 0, a[i]) {
      p[pos++] = j == 0 && i == erri + 1 ? prev - 1 : prev;
    }
    prev = pos;
  }
  output(p);
}
