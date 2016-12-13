#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

int choice(double p, int n, int bit) {
  int r = __builtin_popcount(bit);
  int cnt = 0;
  if (r == 1) {
    cnt = 0;
  } else {
    if (rand() % 1000 < p * 1000 - 0.5) {
      cnt = 0;
    } else {
      cnt = 1 + (rand() % (r - 1));
    }
  }
  REP(i, 0, n) {
    if (bit & 1 << i) {
      if (cnt == 0) { return i; }
      cnt--;
    }
  }
  assert (0);
}

int main(void){
  int n;
  double p1, p2;
  cin >> n >> p1 >> p2;
  VI a(n), b(n);
  int tot_sc = 0;
  REP(i, 0, n) {
    cin >> a[i];
    tot_sc += a[i];
  }
  REP(i, 0, n) {
    cin >> b[i];
    tot_sc += b[i];
  }
  sort(a.begin(), a.end());
  sort(b.begin(), b.end());
  double tot = 0;
  int num_iteration = 160000;
  REP(iteration, 0, num_iteration) {
    int sc = 0;
    int bita = (1 << n) - 1;
    int bitb = bita;
    REP(i, 0, n) {
      int ca = choice(p1, n, bita);
      int cb = choice(p2, n, bitb);
      bita ^= 1 << ca;
      bitb ^= 1 << cb;
      if (a[ca] > b[cb]) {
	sc += a[ca] + b[cb];
      }
    }
    assert (bita == 0);
    assert (bitb == 0);
    if (sc * 2 > tot_sc) {
      tot += 1;
    }
  }
  printf("%.15f\n", tot / num_iteration);
}
