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
/**
 * On-demand Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * The length is up to 2^30. Nodes are created in "on-demand" manner.
 * Header requirement: vector, algorithm
 * Verified by 767 - ghoststudents (https://hoj.hamako-ths.ed.jp/onlinejudge/state/21113)
 */
template<class I, class BiOp = I (*) (I, I)>
class OnDemandSegTree {
  int n;
  I e;
  BiOp op;
  struct node {
    I dat;
    node *left, *right;
    OnDemandSegTree *par;
    node(OnDemandSegTree *par) {
      this->dat = par->e;
      this->left = this->right = NULL;
      this->par = par;
    }
    I updateSub(int k, I v, int l, int r) {
      if (r <= k || k + 1 <= l) { return dat; }
      if (k <= l && r <= k + 1) {
	return dat = v;
      }
      if (this->left == nullptr) {
	this->left = new node(par);
      }
      if (this->right == nullptr) {
	this->right = new node(par);
      }
      I vl = this->left->updateSub(k, v, l, (l + r) / 2);
      I vr = this->right->updateSub(k, v, (l + r) / 2, r);
      return dat = par->op(vl, vr);
    }
  /* l,r are for simplicity */
    I querySub(int a, int b, int l, int r) {
      // [a,b) and  [l,r) intersects?
      if (r <= a || b <= l) return par->e;
      if (a <= l && r <= b) return dat;
      if (this->left == nullptr) {
	this->left = new node(par);
      }
      if (this->right == nullptr) {
	this->right = new node(par);
      }
      I vl = this->left->querySub(a, b, l, (l + r) / 2);
      I vr = this->right->querySub(a, b, (l + r) / 2, r);
      return par->op(vl, vr);
    }
  } root;
public:
  OnDemandSegTree(BiOp op, I e) :e(e), op(op), root(this) {
    n = 1 << 30;
  }
  /* ary[k] <- v */
  void update(int k, I v) {
    root.updateSub(k, v, 0, n);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) {
    return root.querySub(a, b + 1, 0, n);
  }
};

int main(void){
  int n, q;
  cin >> n >> q;
  OnDemandSegTree<int, plus<int> > st(plus<int>(), 0);
  VI tbl(n, 0);
  st.update(0, n);
  REP(i, 0, q) {
    int ty;
    cin >> ty;
    if (ty == 1) {
      int a, b; cin >> a >> b;
      b--;
      if (tbl[b] < a) {
	st.update(tbl[b], st.query(tbl[b], tbl[b]) - 1);
	st.update(a, st.query(a, a) + 1);
        tbl[b] = max(tbl[b], a);
      }
    } else {
      int c;
      cin >> c;
      // On-demand segment tree works, but it's hard to implement.
      int cnt = st.query(c, (1 << 30) - 1);
      cout << cnt << endl;
    }
  }
}
