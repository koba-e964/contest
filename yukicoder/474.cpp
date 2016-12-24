#include <cassert>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const int mod = 1 << 21;
int residual[mod];
int pow2[mod];

int invmod(int x) {
  int e = mod / 2 - 1;
  int sum = 1;
  int cur = x;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = sum * cur;
    }
    cur = cur * cur;
    e /= 2;
  }
  return sum & (mod - 1);
}


void ll_precompute(void) {
  residual[0] = 1;
  pow2[0] = 0;
  REP(i, 1, mod) {
    int r = i;
    int p = 0;
    while (r % 2 == 0) {
      r /= 2;
      p++;
    }
    residual[i] = residual[i - 1] * r;
    pow2[i] = pow2[i - 1] + p;
  }
}

ll h_partial(int x, int y) {
  if (x + y - 1 >= mod) {
    return 0;
  }
  if (y == 0) {
    return 0;
  }
  int rem = 1;
  int p2 = pow2[x + y - 1] - pow2[x] - pow2[y - 1];
  assert (p2 >= 0);
  rem = (rem * residual[x + y - 1]) & (mod - 1);
  rem = rem * invmod((residual[x] * residual[y - 1]) & (mod - 1)) & (mod - 1);
  while (p2 > 0) {
    p2--;
    rem = rem * 2;
  }
  return rem & (mod - 1);
}

int solve(int a, int b, int c) {
  if (c % 2 == 0) {
    return 0;
  }
  // H(a, c * H(b, c)) mod 2
  int rem = 1;
  // Computes H(b, c)
  rem = h_partial(b, c);
  rem = (rem * c) & (mod - 1);
  rem = h_partial(a, rem);
  return rem % 2;
}

int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  ll_precompute();
  while (t--) {
    int a, b, c;
    cin >> a >> b >> c;
    cout << solve(a, b, c) << "\n";
  }
}
