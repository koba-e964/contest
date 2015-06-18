#include <cstdlib>
#include <cstring>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
const double EPS=1e-9;

int dn[4];
const int S = 13;

int dp[4 * S + 1][S + 1][S + 1][S + 1][S + 1][2] = {};


int rec(int th, int jh, int a, int b, int c, int d, int w) {
  int &ret = dp[jh][a][b][c][d][w];
  if (ret >= 0) {
    return ret;
  }
  int res = 1;
  if (a + b + c + d == 0) {
    return ret = w ? th >= jh : th > jh;
  }
  if (a > 0) {
    REP(i, 1, 4) {
      if (a == i) {
	res &= rec(jh / 2, th + jh - jh / 2 + i, 0, b, c, d, 1 - w);
      }
      if (a > i) {
	res &= rec(jh, th + i, a - i, b, c, d, 1 - w);
      }
    }
  }
  if (b > 0) {
    REP(i, 1, 4) {
      if (b == i) {
	res &= rec(jh / 2, th + jh - jh / 2 + i, a, 0, c, d, 1 - w);
      }
      if (b > i) {
	res &= rec(jh, th + i, a, b - i, c, d, 1 - w);
      }
    }
  }
  if (c > 0) {
    REP(i, 1, 4) {
      if (c == i) {
	res &= rec(jh / 2, th + jh - jh / 2 + i, a, b, 0, d, 1 - w);
      }
      if (c > i) {
	res &= rec(jh, th + i, a, b, c - i, d, 1 - w);
      }
    }
  }
  if (d > 0) {
    REP(i, 1, 4) {
      if (d == i) {
	res &= rec(jh / 2, th + jh - jh / 2 + i, a, b, c, 0, 1 - w);
      }
      if (d > i) {
	res &= rec(jh, th + i, a, b, c, d - i, 1 - w);
      }
    }
  }
  return ret = 1 - res;
}


int main(void){
  REP(i, 0, 4) {
    cin >> dn[i];
  }
  memset(dp, -1, sizeof(dp));
  cout << (rec(0, 0, dn[0], dn[1], dn[2], dn[3], 0) ? "Taro" : "Jiro") << endl;
}
