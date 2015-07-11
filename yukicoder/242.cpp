#include <iostream>
#include <cstdio>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

int tbl[12] = {0x1f, 0x3e0, 0x7c00, 0xf8000, 0x1f00000,
	       0x108421, 0x210842, 0x421084, 0x842108, 0x1084210,
	       0x1041041, 0x111110};

double prob(int n, int p) {
  double sum = 1;
  REP(i, 0, p) {
    sum *= n - i;
    sum /= 99 - i;
  }
  return sum;
}

double p[1 << 12];

int main(void){
  int n;
  cin >> n;
  double sum = 0;
  REP(bits, 0, 1 << 12) {
    int acc = 0;
    REP(i, 0, 12) {
      if (bits & 1 << i) {
	acc |= tbl[i];
      }
    }
    int pop = __builtin_popcount(acc);
    p[bits] = prob(n, pop);
  }
  for (int i = 4095; i >= 0; --i) {
    REP(j, i + 1, 1 << 12) {
      if (i & ~j) continue;
      p[i] -= p[j];
    }
  }
  REP(i, 0, 1 << 12) {
    sum += __builtin_popcount(i) * p[i];
  }
  printf("%.9f\n", sum);
}
