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

template<class T>
class TwoBIT {
  BIT<T, plus<T> > bit0, bit1;
public:
  TwoBIT(int n): bit0(n + 1), bit1(n + 1) {}
  // [l, r)
  void add_interval(int l, int r, T x) {
    bit1.add(l + 1, x);
    bit1.add(r + 1, -x);
    bit0.add(l + 1, -x * (l + 1));
    bit0.add(r + 1, x * (r + 1));
  }
  // [l, r)
  T sum_interval(int l, int r) {
    return bit1.accum(r + 1) * (r + 1) + bit0.accum(r + 1)
      - bit1.accum(l + 1) * (l + 1) - bit0.accum(l + 1);
  }
};
