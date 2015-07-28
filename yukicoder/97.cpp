#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

unsigned xor128_x = 123456789, xor128_y = 362436069, xor128_z = 521288629, xor128_w = 88675123;
unsigned xor128() {
	unsigned t = xor128_x ^ (xor128_x << 11);
	xor128_x = xor128_y; xor128_y = xor128_z; xor128_z = xor128_w;
	return xor128_w = xor128_w ^ (xor128_w >> 19) ^ (t ^ (t >> 8));
}
void generateA(int N, int A[]) {
    for(int i = 0; i < N; ++ i)
        A[i] = xor128() % 100003;
}

const int M =100005;
int a[M];
int b[M];
int aset[M] = {};
int memo[M];
int imod[M];
const ll mod = 100003;

ll invmod(ll x) {
  ll e = mod - 2;
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

ll solve(const int *as, int q, int n) {
  ll s = mod - 1;
  REP (i, 0, n) {
    if (aset[(s * (ll)imod[q]) % mod]) {
      return s;
    }
    s--;
  }
  return -1;
}

int main(void){
  int n, q;
  cin >> n >> q;
  REP(i, 0, q) {
    cin >> b[i];
  }
  generateA(n, a);
  REP(i, 0, mod) {
    memo[i] = -1;
  }
  REP(i, 1, mod) {
    imod[i] = invmod(i);
  }
  REP(i, 0, n) {
    aset[a[i]] = 1;
  }
  memo[0] = 0;
  REP(i, 0, q) {
    if (memo[b[i]] >= 0) {
      cout << memo[b[i]] << endl;
      continue;
    }
    int sol = solve(aset, b[i], n);
    if (sol < 0) {
      ll ma = 0;
      REP(j, 0, n) {
	ma = max(ma, ((ll)a[j] * (ll)b[i]) % mod);
      }
      sol = ma;
    }
    cout << (memo[b[i]] = sol) << endl;
  }
}
