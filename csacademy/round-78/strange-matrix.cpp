#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Header requirement: vector, algorithm
 * Verified by ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
 *             AGC009-C (http://agc009.contest.atcoder.jp/submissions/1461012)
 */
template<class I, class BiOp = I (*) (I, I)>
class SegTree {
  int n;
  std::vector<I> dat;
  BiOp op;
  I e;
public:
  SegTree(int n_, BiOp op, I e) : op(op), e(e) {
    n = 1;
    while (n < n_) { n *= 2; } // n is a power of 2
    dat.resize(2 * n);
    for (int i = 0; i < 2 * n - 1; i++) {
      dat[i] = e;
    }
  }
  /* ary[k] <- v */
  void update(int k, I v) {
    k += n - 1;
    dat[k] = v;
    while (k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[2 * k + 1], dat[2 * k + 2]);
    }
  }
  void update_array(int k, int len, const I *vals) {
    for (int i = 0; i < len; ++i) {
      update(k + i, vals[i]);
    }
  }
  /*
    Updates all elements. O(n)
   */
  void update_all(const I *vals, int len) {
    for (int k = 0; k < std::min(n, len); ++k) {
      dat[k + n - 1] = vals[k];
    }
    for (int k = std::min(n, len); k < n; ++k) {
      dat[k + n - 1] = e;
    }
    for (int b = n / 2; b >= 1; b /= 2) {
      for (int k = 0; k < b; ++k) {
	dat[k + b - 1] = op(dat[k * 2 + b * 2 - 1], dat[k * 2 + b * 2]);
      }
    }
  }
  /* http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
  I querySub(int a, int b) const {
    I left = e;
    I right = e;
    a += n - 1;
    b += n - 1;
    while (a < b) {
      if ((a & 1) == 0) {
	left = op(left, dat[a]);
      }
      if ((b & 1) == 0) {
	right = op(dat[b - 1], right);
      }
      a = a / 2;
      b = (b - 1) / 2;
    }
    return op(left, right);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) const {
    return querySub(a, b + 1);
  }
};

const int inf = 1.05e9;

struct pmat {
    vector<VI> operator()(const vector<VI> &a, const vector<VI> &b) const {
        int n = a.size();
        vector<VI> ret(n, VI(n, -inf));
        REP(i, 0, n) {
            REP(j, 0, n) {
                REP(k, 0, n) {
                    ret[i][j] = max(ret[i][j], a[i][k] + b[k][j]);
                }
            }
        }
        return ret;
    }
};


SegTree<vector<VI>,pmat> *seg;
int a[5][21000];
int n, m, q;

void update(int c) {
    vector<VI> elem(n, VI(n, -inf));
    REP(i, 0, n) {
        REP(delta, -1, 2) {
            int nx = delta + i;
            if (nx < 0 || nx >= n) continue;
            elem[nx][i] = a[i][c];
        }
    }
    seg->update(c, elem);
}


int main(void) {
    cin >> n >> m >> q;
    REP(i, 0, n) REP(j, 0, m) cin >> a[i][j];
    vector<VI> unit(n, VI(n, -inf));
    REP(i, 0, n) unit[i][i] = 0;
    seg = new SegTree<vector<VI>,pmat>(m, pmat(), unit);
    REP(i, 0, m) update(i);
    REP(_, 0, q) {
        int r, c, t;
        cin >> r >> c >> t;
        r--, c--;
        a[r][c] = t;
        update(c);
        vector<VI> res = seg->query(0, m - 1);
        int ma = -inf;
        REP(i, 0, n) REP(j, 0, n) ma = max(ma, res[i][j]);
        cout << ma << endl;
    }
}
