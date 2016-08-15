#include <vector>
#include <string>
#include <algorithm>
#include <iostream>
#include <cstdio>
#include <cassert>
#include <cstdlib>

const int K=1001;
const int MOD=1000000007;
inline int mod(int x,int y){
	return ((x%y)+y)%y;
}

using namespace std;
typedef long long ll;
#define REP(i, s, t) for (int i = int(s); i < int(t); ++i)

// Calculate a * b mod (x^k - x^{k - 1} - ... - 1).
int k, n;

vector<ll> mul(const vector<ll> &a, const vector<ll> &b) {
  vector<ll> ret(2 * k);
  REP(i, 0, k) {
    REP(j, 0, k) {
      ret[i + j] += a[i] * b[j];
      ret[i + j] %= MOD;
    }
  }
  // mod (x^{k + 1} - 2 x^k + 1)
  for (int i = 2 * k - 1; i >= k + 1; --i) {
    ret[i - 1] += ret[i] * 2 % MOD;
    ret[i - 1] %= MOD;
    ret[i - k - 1] += MOD - ret[i];
    ret[i - k - 1] %= MOD;
    ret[i] = 0;
  }
  REP(i, 0, k) {
    ret[i] += ret[k];
    ret[i] %= MOD;
  }
  return vector<ll>(ret.begin(), ret.begin() + k);
}

int main(void){
  cin>>k>>n;
  vector<ll> cur(k, 0), sum(k, 0);
  cur[1] = 1;
  sum[0] = 1;
  // (1 1 1 ... 1) * (0 1 0 ... 0)^(n - 1)
  n--;
  while (n > 0) {
    if (n % 2) {
      sum = mul(sum, cur);
    }
    cur = mul(cur, cur);
    n /= 2;
  }
  ll s = 0;
  REP(i, 0, k) {
    s += sum[i];
    s %= MOD;
  }
  cout << s << endl;
}
