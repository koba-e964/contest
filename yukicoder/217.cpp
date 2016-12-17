#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

vector<VI> odd(int n) {
  vector<VI> tbl(n, VI(n, -100));
  REP(i, 0, n) {
    int x = (2 * i) % n;
    int y = (n / 2 - i + n) %  n;
    REP(j, 0, n) {
      tbl[(x - j + n) % n][(y + j) % n] = 1 + n * i + j;
    }
  }
  return tbl;
}

vector<VI> multiple4(int n) {
  vector<VI> tbl(n, VI(n, -100));
  REP(i, 0, n) {
    REP(j, 0, n) {
      int a = i & 3;
      int b = j & 3;
      if (0x9669 & 1 << (4 * a + b)) { // on a diagonal
	tbl[i][j] = 1 + i * n + j;
      } else {
	tbl[n - 1 - i][n - 1 - j] = 1 + i * n + j;
      }
    }
  }
  return tbl;
      
}

vector<VI> rest(int n) {
  vector<VI> sub = odd(n / 2);
  int l[2][2] = {
    {0, -3},
    {-2, -1}};
  int u[2][2] = {
    {-3, 0},
    {-2, -1}};
  int x[2][2] = {
    {-3, 0},
    {-1, -2}};
  int(* mat[3])[2] = {l, u, x};
  vector<vector<int> > pow(n / 2, VI(n / 2));
  int m = n / 4; // n / 2 = 2m + 1
  REP(i, 0, m + 1) {
    REP(j, 0, n / 2) {
      pow[i][j] = 0;
    }
  }
  REP(j, 0, n / 2) {
    pow[m + 1][j] = 1;
  }
  REP(i, m + 2, n / 2) {
    REP(j, 0, n / 2) {
      pow[i][j] = 2;
    }
  }
  swap(pow[m][m], pow[m + 1][m]);
  vector<VI> tbl(n, VI(n));
  REP(i, 0, n / 2) {
    REP(j, 0, n / 2) {
      REP(k, 0, 2) {
	REP(p, 0, 2) {
	  tbl[2 * i + k][2 * j + p] = 4 * sub[i][j] + mat[pow[i][j]][k][p];
	}
      }
    }
  }
  return tbl;
}

int main(void){
  int n;
  cin >> n;
  vector<VI> tbl;
  if (n % 2 == 1) {
    tbl = odd(n);
  } else if (n % 4 == 0) {
    tbl = multiple4(n);
  } else {
    tbl = rest(n);
  }
  REP(i, 0, n) {
    REP(j, 0, n) {
      cout << tbl[i][j] << (j == n - 1 ? "\n" : " ");
    }
  }
}
