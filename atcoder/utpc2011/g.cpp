#include <algorithm>
#include <cassert>
#include <cmath>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;


// a should be sorted in the reverse order
ll full_sol(const VL &a) {
  int n = a.size();
  if (n < 6) {
    return 0;
  }
  ll ma = 0;
  {
    // (i, j) where j >= i + 3
    ll st = -1;
    VL dat(n, -1);
    REP(i, 0, n - 2) {
      if (a[i] < a[i + 1] + a[i + 2]) {
	dat[i] = a[i] + a[i + 1] + a[i + 2];
      }
      if (i >= 3) {
	st = max(st, dat[i - 3]);
      }
      if (st >= 0 && dat[i] >= 0) {
	ma = max(ma, st + dat[i]);
      }
    }
  }
  REP(i, 0, n - 5) {
    // j == i + 1
    int j = i + 1;
    VL rest(a.begin() + i + 2, a.begin() + i + 6);
    sort(rest.begin(), rest.end());
    ll resttot = 0;
    REP(i, 0, 4) {
      resttot += rest[i];
    }
    sort(rest.begin(), rest.end());
    bool ok = false;
    do {
      if (a[i] < rest[0] + rest[1] && a[j] < rest[2] + rest[3]) {
	ok = true;
	break;
      }
    } while (next_permutation(rest.begin(), rest.end()));
    if (ok) {
      ma = max(ma, a[i] + a[j] + resttot);
    }
  }
  REP(i, 0, n - 5) {
    // j == i + 2
    int j = i + 2;
    VL rest(a.begin() + j + 1, a.begin() + j + 4);
    sort(rest.begin(), rest.end());
    ll resttot = 0;
    REP(i, 0, 3) {
      resttot += rest[i];
    }
    bool ok = false;
    do {
      if (a[i] < rest[0] + a[i + 1] && a[j] < rest[1] + rest[2]) {
	ok = true;
	break;
      }
    } while (next_permutation(rest.begin(), rest.end()));
    if (ok) {
      ma = max(ma, a[i] + a[j] + resttot + a[i + 1]);
    }
  }
  return ma;
}


int main(void) {
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
  }
  sort(a.rbegin(), a.rend());
  cout << full_sol(a) << endl;
}
