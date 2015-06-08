#include <vector>

/*
 * Header requirement: vector
 */
class GF2Matrix {
private:
  std::vector<std::vector<bool> > mat;
  int n, m;
public:
  GF2Matrix(int n, int m) : n(n), m(m), mat(n) {
    for (int i = 0; i < n; ++i) {
      mat[i] = std::vector<bool>(m);
    }
  }
  std::vector<bool> &operator[](int i) {
    return mat[i];
  }
  const std::vector<bool> &operator[](int i) const {
    return mat[i];
  }
  /* Returns the rank of this matrix. */
  int eliminate(void) {
    int r = 0;
    for (int c = 0; c < m; ++c) {
      int r1 = -1;
      for (int i = r; i < n; ++i) {
	if (mat[i][c]) {
	  r1 = i;
	  break;
	}
      }
      if (r1 == -1) {
	continue;
      }
      std::swap(mat[r], mat[r1]);
      for (int i = 0; i < n; ++i) {
	if (i != r && mat[i][c]) {
	  for (int j = c; j < m; ++j) {
	    mat[i][j] = mat[i][j] ^ mat[r][j];
	  }
	}
      }
      ++r;
    }
    return r;
  }
};
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#include <iostream>

using namespace std;

int main(void){
  int n;
  cin >> n;
  GF2Matrix mat(n, n + 1);
  REP(i, 0, n) {
    int d;
    cin >> d;
    int r1 = (i + d) % n;
    int r2 = (i - d % n + n) % n;
    mat[r1][i] = 1;
    mat[r2][i] = 1;
  }
  REP(i, 0, n) {
    int w;
    cin >> w;
    mat[i][n] = 1 - w;
  }
  int r = mat.eliminate();
  bool ok = 0;
  if (r >= 1) {
    REP(i, 0, n) {
      ok |= mat[r - 1][i];
    }
    ok |= !mat[r - 1][n];
  }
  cout << (ok ? "Yes" : "No") << endl;
}
