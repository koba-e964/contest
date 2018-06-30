#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;


const int N = 262144;

int bit[N];
int accum(int v) {
  int tot = 0;
  while (v > 0) {
    tot += bit[v];
    v &= v - 1;
  }
  return tot;
}

void update(int idx, int x) {
  while (idx < N) {
    bit[idx] += x;
    idx += idx & (-idx);
  }
}


int main(void) {
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i] -= i + 1;
  }
  // Coord compression
  map<ll, int> tbl;
  {
    vector<ll> inv_tbl;
    set<int> vals;
    vals.insert(0);
    REP(i, 0, n) {
      if (a[i] >= 0) {
      vals.insert(a[i]);
      }
    }
    inv_tbl = vector<ll>(vals.begin(), vals.end());
    sort(inv_tbl.begin(), inv_tbl.end());
    REP(i, 0, inv_tbl.size()) {
      tbl[inv_tbl[i]] = i;
    }
  }
  int m = tbl.size();
  REP(i, 0, n) {
    if (tbl.count(a[i])) {
      int idx = tbl[a[i]];
      int lo = idx;
      int hi = m;
      int val = accum(idx + 1);
      while (hi - lo > 1) {
	int mid = (hi + lo) / 2;
	if (accum(mid + 1) == val) {
	  lo = mid;
	} else {
	  hi = mid;
	}
      }
      update(lo + 2, -1);
      update(idx + 1, 1);
    }
  }
  cout << n - accum(m) << endl;
}
