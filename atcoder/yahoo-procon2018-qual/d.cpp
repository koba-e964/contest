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


const int B = 2050;
ll comb[B][B];
// ll comb_acc[B][B];

void add(ll &x, ll y) {
  x = (x + y) % mod;
}

void init(void) {
  comb[0][0] = 1;
  REP(i, 1, B) {
    REP(j, 0, B) {
      add(comb[i][j], comb[i - 1][j]);
      if (j > 0) add(comb[i][j], comb[i - 1][j - 1]);
    }
  }
  REP(i, 0, B) {
    REP(j, 1, B) {
      add(comb[i][j], comb[i][j - 1]);
    }
  }
  if (0) {
    cerr<<"comb:"<<endl;
    REP(i, 0, 4) {
      REP(j, 0, 5) {
	cerr << " " <<comb[i][j];
      }
      cerr<<endl;
    }
  }
}



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  int n, k, yy;
  VI x;
  vector<VI> a;
  {
    int xx;
    cin >> n >> k >> xx >> yy;
    yy ^= xx;
    x = VI(n);
    a = vector<VI>(k, VI(k));
    REP(i, 0, n) cin >> x[i];
    REP(i, 0, k) {
      REP(j, 0, k) {
	cin >> a[i][j];
	a[i][j] ^= xx;
      }
    }
  }
  VI freq(2048);
  REP(i, 0, n) {
    freq[x[i]] += 1;
  }
  bool ok = 1;
  {
    VI b(k);
    b[0] = 0;
    REP(i, 1, k) {
      b[i] = b[0] ^ a[0][i];
    }
    // compat?
    REP(i, 0, k) {
      REP(j, 0, k) {
	int diff = a[i][j] ^ b[i] ^ b[j];
	if (diff == 0 || diff == yy) {
	  continue;
	}
	ok = false;
      }
    }
  }
  if (not ok) {
    cout << 0 << endl;
    return 0;
  }
  ll tot = 0;
  REP(bits, 0, 2048) {
    int an = bits ^ yy;
    if (bits > an) continue;
    VI b(k);
    b[0] = bits;
    REP(i, 1, k) {
      b[i] = b[0] ^ a[0][i];
      b[i] = min(b[i], b[i] ^ yy);
    }
    VI bf(2048);
    REP(i, 0, k) {
      bf[b[i]] += 1;
    }
    ll prod = 1;
    REP(i, 0, 2048) {
      if (i > (i ^ yy)) continue;
      int p = bf[i];
      int q = freq[i];
      int r = freq[i ^ yy];
      int ma = min(q, p);
      int mi = max(0, p - r);
      if (ma < mi) {
	prod = 0;
	break;
      }
      ll tmp = comb[p][ma] - (mi == 0 ? 0 : comb[p][mi - 1]) + mod;
      prod = prod * (tmp % mod) % mod;
    }
    add(tot, prod);
  }
  cout << tot << endl;
}
