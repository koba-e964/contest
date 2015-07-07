#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

const int N = 5000001;
int tbl[N];

int dset(int v) {
  int bits = 0;
  while (v > 0) {
    bits |= 1 << (v % 10);
    v /= 10;
  }
  return bits;
}

void solve(int bits) {
  tbl[0] = tbl[1] = 0;
  
  REP(i, 2, N) {
    tbl[i] = dset(i);
  }
  REP(i, 2, sqrt(N) + 1) {
    if (tbl[i]) {
      REP(j, 2, (N + i - 1) / i) {
	tbl[i * j] = 0;
      }
    }
  }
  int ma = -1;
  REP(i, 1, N) {
    int acc = 0;
    int j = i;
    for (; j < N; ++j) {
      if (tbl[j] & ~bits) {
	break;
      }
      acc |= tbl[j];
      if (acc == bits) {
	ma = max(ma, j - i);
      }
    }
    i = j;
  }
  cout << ma << endl;
}

int main(void){
  int sn;
  cin >> sn;
  int bits = 0;
  REP(i, 0, sn) {
    int y;
    cin >> y;
    bits |= 1 << y;
  }
  solve(bits);
}
