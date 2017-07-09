#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll inf = 1e15;

// Reference: http://mmxsrup.hatenablog.com/category/%E9%81%85%E5%BB%B6%E8%A9%95%E4%BE%A1%E3%82%BB%E3%82%B0%E3%83%A1%E3%83%B3%E3%83%88%E6%9C%A8
// with a little modification (add, min)
//(1)区間に一様加算 (2)区間の合計値
struct segtree{
public:
    const int SIZE = 1 << 18;
    //seg:区間の合計値 lazy:区間に対して、加える値でまだ遅延しているもの
    vector<ll> seg, lazy;//segは欲しい情報 lazyは区間に対する一様な処理を示すもの
    segtree():seg(SIZE * 2), lazy(SIZE * 2){}
    void lazy_evaluate(int k, int l, int r){//遅延情報の適用方法
        if(lazy[k] != 0){
            seg[k] += lazy[k];//区間[l,r)にすべて同じ値を追加することになっていて、segには合計値が入っているので、加える値を足す
            if(r  - l > 1){
                lazy[k * 2 + 1] += lazy[k];//遅延を左の子に伝搬
                lazy[k * 2 + 2] += lazy[k];//遅延を右の子に伝搬
            }
            lazy[k] = 0;//ノードkは伝搬完了
        }
    }
    void update(int a, int b, int k, int l, int r, ll x){
        lazy_evaluate(k, l, r);
        if(r <= a || b <= l) return;
        if(a <= l && r <= b){
            lazy[k] += x; //加える
            lazy_evaluate(k, l, r);
        }else{
            update(a, b, k * 2 + 1, l, (l + r) / 2, x);
            update(a, b, k * 2 + 2, (l + r) / 2, r, x);
            seg[k] = min(seg[k * 2 + 1], seg[k * 2 + 2]); //区間の合計
        }
    }  
    ll query(int a, int b, int k, int l, int r){
        lazy_evaluate(k, l, r);
        if(r <= a || b <= l) return inf;//合計に影響のないもの
        if(a <= l && r <= b) return seg[k];
        ll x = query(a, b, k * 2 + 1, l, (l + r) / 2);
        ll y = query(a, b, k * 2 + 2, (l + r) / 2, r);
        return min(x, y); //左右の合計を
    }
    //update(a,b,x) := [a,b)を全てxを加える
    void update(int a, int b, ll x){update(a, b, 0, 0, SIZE, x);}
    //query(a,b) := [a,b)に対する合計値を求める
    ll query(int a, int b){return query(a, b, 0, 0, SIZE);}
};


/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * Header requirement: vector, algorithm
 * Verified by AtCoder ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
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
  /* l,r are for simplicity */
  I querySub(int a, int b, int k, int l, int r) const {
    // [a,b) and  [l,r) intersects?
    if (r <= a || b <= l) return e;
    if (a <= l && r <= b) return dat[k];
    I vl = querySub(a, b, 2 * k + 1, l, (l + r) / 2);
    I vr = querySub(a, b, 2 * k + 2, (l + r) / 2, r);
    return op(vl, vr);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) const {
    return querySub(a, b + 1, 0, 0, n);
  }
};


struct cordis {
  int x, y, z, w;
  // x: giving to left, y: receivable from right, z: absorbed
  cordis(): x(-1), y(-1), z(-1), w(-1) {}
  cordis(int x, int y, int z, int w): x(x), y(y), z(z), w(w) {
  }
};

ostream& operator<<(ostream &os, cordis c) {
  os << c.x << " " << c.y << " " << c.z << " " << c.w;
  return os;
}
struct cop {
  cordis operator()(cordis a, cordis b) const {
    if (a.x == -1) {
      return b;
    }
    if (b.x == -1) {
      return a;
    }
    int d = min(a.y, b.x);
    if (a.z && b.z) {
      return cordis(0, b.y + a.y, 1, 0);
    }
    if (b.z) {
      return cordis(a.x, a.y + b.y, 0, a.w + b.w);
    }
    if (a.z) {
      int e = min(b.x, a.y);
      return cordis(b.x - e, b.y, 0, a.w + b.w);
    }
    return cordis(a.x, b.y, 0, a.w + b.w + b.x - d);
  }
};

int main(void){
  int n, m;
  cin >> n >> m;
  VI a(n), x(m), y(m);
  VI f(n + 1, 0);
  REP(i, 0, n) {
    cin >> a[i];
    f[a[i]] += 1;
  }
  REP(i, 0, m) {
    cin >> x[i] >> y[i];
    x[i]--;
  }
  VI veil(n + 1, 0);
  REP(i, 1, n + 1) {
    REP(j, 0, min(f[i], i)) {
      veil[i - j] += 1;
    }
  }
  int uncover = 0;
  REP(i, 1, n + 1) {
    uncover += veil[i] == 0 ? 1 : 0;
  }
  REP(i, 0, m) {
    int olda = a[x[i]];
    a[x[i]] = y[i];
    f[olda] -= 1;
    f[y[i]] += 1;
    if (y[i] != olda) {
      int stake1 = olda - f[olda];
      if (stake1 >= 1) {
	veil[stake1] -= 1;
	if (veil[stake1] == 0) {
	  uncover += 1;
	}
      }
      int stake2 = y[i] - f[y[i]] + 1;
      if (stake2 >= 1) {
	veil[stake2] += 1;
	if (veil[stake2] == 1) {
	  uncover -= 1;
	}
      }
    }
    cout << uncover << endl;
  }
}
