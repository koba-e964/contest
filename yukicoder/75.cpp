#include <vector>
/*
 * Header requirement: vector
 */
template<class T = double>
class Matrix {
private:
  std::vector<std::vector<T> > mat;
  int n, m;
public:
  Matrix(int n, int m) : n(n), m(m), mat(n) {
    for (int i = 0; i < n; ++i) {
      mat[i] = std::vector<T>(m);
    }
  }
  std::vector<T> &operator[](int i) {
    return mat[i];
  }
  const std::vector<T> &operator[](int i) const {
    return mat[i];
  }
  /* Returns the rank of this matrix. */
  int eliminate(void) {
    int r = 0;
    for (int c = 0; c < m; ++c) {
      int r1 = -1;
      for (int i = r; i < n; ++i) {
	if (r1 == -1 || std::max(mat[i][c], -mat[i][c]) > std::max(mat[r1][c], -mat[r1][c])) {
	  r1 = i;
	  break;
	}
      }
      if (r1 == -1) {
	continue;
      }
      std::swap(mat[r], mat[r1]);
      for (int i = 0; i < m; i++) {
	if (i == c) {
	  continue;
	}
	mat[r][i] /= mat[r][c];
      }
      mat[r][c] = 1;
      for (int i = 0; i < n; ++i) {
	if (i != r) {
	  T fact = mat[i][c];
	  for (int j = c; j < m; ++j) {
	    mat[i][j] -= mat[r][j] * fact;
	  }
	}
      }
      ++r;
    }
    return r;
  }
};
#include <iostream>
#include <cassert>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;


int main(void){
  int k;
  cin >> k;
  Matrix<> mat(k, k + 1);
  REP(i, 0, k) {
    mat[i][i] = -1;
    mat[i][k] = 1;
    REP(j, 1, 7) {
      if (i + j <= k) {
	mat[i][i + j] += i + j == k ? 0 : 1.0 / 6;
      } else {
	mat[i][0] += 1.0 / 6;
      }
    }
  }
  int r = mat.eliminate();
  assert (r == k);
  cout << -mat[0][k] << endl;
}
