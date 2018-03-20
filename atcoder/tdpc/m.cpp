#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
const ll mod = 1e9 + 7;

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

vector<VL> mul(const vector<VL> &a, const vector<VL> &b) {
  int n = a.size();
  vector<VL> ret(n, VL(n, 0));
  REP(i, 0, n) {
    REP(j, 0, n) {
      REP(k, 0, n) {
	add(ret[i][j], a[i][k] * b[k][j]);
      }
    }
  }
  return ret;
}


const int R = 16;
ll dp[R][R][1 << R];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, r;
  cin >> h >> r;
  vector<VI> g(r, VI(r));
  REP(i, 0, r) {
    REP(j, 0, r) {
      cin >> g[i][j];
    }
  }
  REP(i, 0, r) {
    dp[i][i][1 << i] = 1;
  }
  REP(i, 0, r) {
    REP(bits, 0, 1 << r) {
      if ((bits & 1 << i) == 0) continue;
      REP(j, 0, r) {
	if ((bits & 1 << j) == 0) continue;
	REP(k, 0, r) {
	  if (bits & 1 << k) continue;
	  if (g[j][k] == 0) continue;
	  add(dp[i][k][bits | 1 << k], dp[i][j][bits]);
	}
      }
    }
  }
  vector<VL> mat(r, VL(r, 0));
  REP(i, 0, r) {
    REP(j, 0, r) {
      REP(bits, 0, 1 << r) {
	add(mat[i][j], dp[i][j][bits]);
      }
    }
  }
  vector<VL> prod(r, VL(r, 0));
  REP(i, 0, r) {
    prod[i][i] = 1;
  }
  vector<VL> cur(mat);
  while (h > 0) {
    if (h % 2 == 1) {
      prod = mul(prod, cur);
    }
    cur = mul(cur, cur);
    h /= 2;
  }
  cout << prod[0][0] << endl;
}
