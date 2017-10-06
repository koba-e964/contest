#include <cmath>
#include <cassert>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int N = 200;
double fact[N];

int main(void){
  int f, n, k;
  cin >> f >> n >> k;
  assert (f == 4 || f == 6 || f == 8 || f == 12 || f == 20);
  assert (1 <= n);
  assert (n <= 100);
  assert (1 <= k);
  assert (k <= n);
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i;
  }
  double sum = 0;
  REP(i, 1, f + 1) {
    // k-th value is i
    REP(j, 0, k) {
      // we have j values in range i + 1 .. f
      REP(l, 1, n + 1) {
	// we have l values that are i
	if (j < k && j + l >= k && j + l <= n) {
	  double tmp = (j == 0 ? 1 : pow(f - i, j))
	    * (l == 0 ? 1 : pow(i - 1, n - j - l));
	  tmp /= pow(f, n);
	  tmp *= fact[n] / fact[j] / fact[l] / fact[n - j - l];
	  sum += tmp * i;
	}
      }
    }
  }
  printf("%.15f\n", sum);
}
