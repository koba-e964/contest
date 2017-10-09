#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int N = 2600;

// xor
ll bit[N][N];

ll accum(int x, int y) {
  ll tot = 0;
  while (x > 0) {
    int yc = y;
    while (yc > 0) {
      tot ^= bit[x][yc];
      yc &= yc - 1;
    }
    x &= x - 1;
  }
  return tot;
}

void update(int x, int y, ll val) {
  if (x <= 0 || y <= 0) {
    return;
  }
  while (x < N) {
    int yc = y;
    while (yc < N) {
      bit[x][yc] ^= val;
      yc += yc & (-yc);
    }
    x += x & (-x);
  }
}

ll get_hash(int r1, int c1, int r2, int c2) {
  ll tmp = ll(r1) << 48 | ll(r2) << 36 | ll(c1) << 24 | ll(c2) << 12;
  tmp += r1 * (r2 + c2);
  tmp += (r1 + c1 + c2) * r2;
  tmp += ll(r1 + c1) << (c1 & 63);
  tmp += ll(r1 + c1) << (c2 & 63);
  return tmp;
}


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, m, q;
  cin >> n >> m >> q;
  REP(ivy_wreath, 0, q) {
    int t, r1, c1, r2, c2;
    cin >> t >> r1 >> c1 >> r2 >> c2;
    if (t == 3) {
      if (0) {
	cerr << "tbl:\n";
	REP(i, 1, n + 1) {
	  REP(j, 1, m + 1) {
	    cerr << " " << accum(i, j);
	  }
	  cerr << endl;
	}
      }
      ll p1 = accum(r1, c1);
      ll p2 = accum(r2, c2);
      cout << (p1 == p2 ? "Yes" : "No") << "\n";
    } else {
      // Note that because we are using xor, t == 1 and t == 2 are identical.
      ll h = get_hash(r1, c1, r2, c2);
      update(r2 + 1, c2 + 1, h);
      update(r2 + 1, c1, h);
      update(r1, c2 + 1, h);
      update(r1, c1, h);
    }
  }
}
