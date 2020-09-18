#include <cassert>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int B = 317;
const int N = 100489;
const int K = N / B; // 317
ll a[N];
ll delay[B];
ll mi[B];

void update_naive(int l, int r, ll c) {
  REP(i, l, r) {
    a[i] += c;
  }
  int lb = l / K;
  if (lb >= B) return;
  ll m = a[K * lb];
  REP(i, 0, K) {
    m = min(m, a[K * lb + i]);
  }
  mi[lb] = m;
}

int main(void){
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  assert (1 <= n && n <= 100000);
  REP(i, 0, n) {
    cin >> a[i];
    assert (a[i] >= -1e10 && a[i] <= 1e10);
  }
  REP(i, 0, B) {
    ll m = a[i * K];
    REP(j, 1, K) {
      m = min(m, a[i * K + j]);
    }
    mi[i] = m;
  }
  int q;
  cin >> q;
  assert (1 <= q && q <= 100000);
  REP(i, 0, q) {
    int k, l, r;
    ll c;
    assert (cin >> k >> l >> r >> c);
    assert (k == 1 || k == 2);
    assert (1 <= l && l <= r && r <= n);
    assert (-10000 <= c && c <= 10000);
    l--; // [l,r)
    int l_boundary = (l + K - 1) / K;
    int r_boundary = r / K;
    // [l,r) = [l,K*l_boundary) + [K*l_boundary,K*r_boundary) + [K*r_boundary,r)
    if (k == 1) { // update interval
      if (l_boundary > r_boundary) {
	update_naive(l, r, c);
      } else {
	update_naive(l, K * l_boundary, c);
	REP(i, l_boundary, r_boundary) {
	  delay[i] += c;
	}
	update_naive(K * r_boundary, r, c);
      }
    }
    if (k == 2) {
      ll m = 1e16;
      if (l_boundary > r_boundary) {
	REP(i, l, r) {
	  m = min(m, a[i] + delay[i / K]);
	}
      } else {
	REP(i, l, K * l_boundary) {
	  m = min(m, a[i] + delay[i / K]);
	}
	REP(i, l_boundary, r_boundary) {
	  m = min(m, mi[i] + delay[i]);
	}
	REP(i, K * r_boundary, r) {
	  m = min(m, a[i] + delay[i / K]);
	}
      }
      cout << m << endl;
    }
  }
}
