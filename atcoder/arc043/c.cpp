#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


/**
 * Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
 * T is a commutative monoid. Indices are 1 .. n.
 * Verified by AtCoder ARC043 C (http://arc043.contest.atcoder.jp/submissions/985157)
 */
template <class T, class Op> class BIT {
private:
  int n;
  std::vector<T> ary;
  Op op;
  T e;
public:
  BIT(int n, Op op = Op(), T e = T()) : op(op), e(e) {
    assert (n > 0);
    while(n != (n & (-n))) { // find the least power of 2 >= n
      n += n & (-n);
    }
    this->n = n;
    ary = std::vector<T>(n + 1);
    for(int i = 0; i <= n; i++) {
      ary[i] = e;
    }
  }
  /**
   * gets the sum in [1 .. idx]
   * @param idx
   * @return sum
   */
  T accum(int idx) {
    T sum = e;
    while(idx > 0) {
      sum = op(sum, ary[idx]);
      idx &= idx - 1;
    }
    return sum;
  }
  /**
   * performs data[idx] += val;
   */
  void add(int idx, T val) {
    assert (idx > 0);
    while(idx <= n) {
      ary[idx] = op(ary[idx], val);
      idx += idx & (-idx);
    }
  }
};
 
std::vector<int> perm_inv(const std::vector<int> &p) {
  int len = p.size();
  std::vector<int> ans(len);
  for (int i = 0; i < len; ++i) {
    assert (0 <= p[i] && p[i] < len);
    ans[p[i]] = i;
  }
  return ans;
}

// q after p
std::vector<int> perm_comp(const std::vector<int> &q, const std::vector<int> &p) {
  int n = p.size();
  std::vector<int> ret(n);
  for (int i = 0; i < n; ++i) {
    ret[i] = q[p[i]];
  }
  return ret;
}

ll count_inversion(const VI &perm) {
  int n = perm.size();
  BIT<int, plus<int> > bit(n + 1);
  // b's inversion number
  ll inv_c = 0;
  REP(i, 0, n) {
    inv_c += bit.accum(n) - bit.accum(perm[i] + 1);
    bit.add(perm[i] + 1, 1);
  }
  return inv_c;
}

int main(void){
  int n;
  cin >> n;
  VI a(n), b(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  REP(i, 0, n) {
    cin >> b[i];
    b[i]--;
  }
  b = perm_comp(perm_inv(a), b);
  ll inv_b = count_inversion(b);
  if (inv_b % 2 == 1) {
    cout << -1 << endl;
    return 0;
  }
  BIT<int, plus<int> > bit(n + 1);
  ll half_b = inv_b / 2;

  VI c;
  
  REP(i, 0, n) {
    if (half_b <= 0) {
      break;
    }
    int cost = bit.accum(n) - bit.accum(b[i] + 1);
    if (half_b >= cost) {
      c.push_back(b[i]);
      half_b -= cost;
      bit.add(b[i] + 1, 1);
    } else {
      break;
    }
  }
  sort(c.begin(), c.end());
  int pos = c.size();
  int inspos = -1;
  if (pos < n) {
    inspos = pos - half_b;
    pos++;
    c.insert(c.begin() + inspos, b[pos - 1]);
    REP(i, pos, n) {
      c.push_back(b[i]);
    }
  }
  c = perm_comp(a, c);
  b = perm_comp(a, b);
  assert (count_inversion(perm_comp(perm_inv(a), c)) ==
	  count_inversion(perm_comp(perm_inv(c), b)));
  REP(i, 0, n) {
    cout << c[i] + 1 << (i == n - 1 ? "\n" : " ");
  }
}
