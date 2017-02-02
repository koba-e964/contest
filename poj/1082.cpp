#include <cstdio>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int len(int y, int m) {
  int tbl[12] = {31, 28, 31, 30,
		 31, 30, 31, 31,
		 30, 31, 30, 31};
  int leap = y % 4 == 0 ^ y % 100 == 0 ^ y % 400 == 0;
  return leap && m == 1 ? 29 : tbl[m];
}

int dp[201][12][32];

void next_day(int y, int m, int d, int &ny, int &nm, int &nd) {
  ny = y;
  nm = m;
  nd = d;
  nd += 1;
  if (nd >= len(ny, nm)) {
    nd = 0;
    nm += 1;
  }
  if (nm >= 12) {
    ny += 1;
    nm = 0;
  }
}

bool next_month(int y, int m, int d, int &ny, int &nm, int &nd) {
  ny = y;
  nm = m;
  nd = d;
  nm += 1;
  if (nm >= 12) {
    nm = 0;
    ny += 1;
  }
  return nd < len(ny, nm);
}


int main(void){
  int t;
  scanf("%d", &t);
  for (int y = 2001; y >= 1900; --y) {
    for (int m = 11; m >= 0; --m) {
      for (int d = 31; d >= 0; --d) {
	// Nov 4, 2001
	if (500 * y + 40 * m + d >= 500 * 2001 + 40 * 10 + 3) {
	  dp[y - 1900][m][d] =
	    500 * y + 40 * m + d == 500 * 2001 + 40 * 10 + 3 ? 0 : 1;
	  continue;
	}
	int ny, nm, nd;
	int res = 1;
	next_day(y, m, d, ny, nm, nd);
	res &= dp[ny - 1900][nm][nd];
	if (next_month(y, m, d, ny, nm, nd)) {
	  res &= dp[ny - 1900][nm][nd];
	}
	dp[y - 1900][m][d] = 1 - res;
      }
    }
  }
  while (t--) {
    int y, m, d;
    scanf("%d%d%d", &y, &m, &d);
    y -= 1900;
    m--, d--;
    puts(dp[y][m][d] ? "YES" : "NO");
  }
}
