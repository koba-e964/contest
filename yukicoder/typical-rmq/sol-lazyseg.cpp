#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

// Reference: http://mmxsrup.hatenablog.com/category/%E9%81%85%E5%BB%B6%E8%A9%95%E4%BE%A1%E3%82%BB%E3%82%B0%E3%83%A1%E3%83%B3%E3%83%88%E6%9C%A8
// with a little modification (add, min)
//(1)区間に一様加算 (2)区間の合計値
const int SIZE = 1 << 17;

struct segtree{
public:
  //seg:区間の合計値 lazy:区間に対して、加える値でまだ遅延しているもの
  vector<ll> seg, lazy;//segは欲しい情報 lazyは区間に対する一様な処理を示すもの
  segtree():seg(SIZE * 2), lazy(SIZE * 2, 0){}
  void lazy_evaluate(int k, int l, int r){//遅延情報の適用方法
    if(lazy[k] != 0){
      seg[k] += lazy[k];
      if(r - l > 1){
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
    } else {
      update(a, b, k * 2 + 1, l, (l + r) / 2, x);
      update(a, b, k * 2 + 2, (l + r) / 2, r, x);
      seg[k] = min(seg[k * 2 + 1], seg[k * 2 + 2]); //区間の合計
    }
  }
  void update_single(int a, int k, int l, int r, ll x){
    lazy_evaluate(k, l, r);
    if(r <= a || a + 1 <= l) return;
    if(a <= l && r <= a + 1){
      seg[k] = x;
    }else{
      update_single(a, k * 2 + 1, l, (l + r) / 2, x);
      update_single(a, k * 2 + 2, (l + r) / 2, r, x);
      seg[k] = min(seg[k * 2 + 1], seg[k * 2 + 2]); //区間の合計
    }
  }
  ll query(int a, int b, int k, int l, int r){
    lazy_evaluate(k, l, r);
    if(r <= a || b <= l) return ll(1e16);//合計に影響のないもの
    if(a <= l && r <= b) return seg[k];
    ll x = query(a, b, k * 2 + 1, l, (l + r) / 2);
    ll y = query(a, b, k * 2 + 2, (l + r) / 2, r);
    return min(x, y); //左右の合計を
  }
  //update(a,b,x) := [a,b)を全てxを加える
  void update(int a, int b, ll x){update(a, b, 0, 0, SIZE, x);}
  void update_single(int a, ll x){update_single(a, 0, 0, SIZE, x);}
  //query(a,b) := [a,b)に対する合計値を求める
  ll query(int a, int b){return query(a, b, 0, 0, SIZE);}
};

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);

  int n;
  cin >> n;
  assert (1 <= n && n <= 100000);
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
    assert (a[i] >= -1e10 && a[i] <= 1e10);
  }
  int q;
  cin >> q;
  assert (1 <= q && q <= 100000);
  segtree st;
  REP(i, 0, n) {
    st.update_single(i, a[i]);
  }
  REP(i, 0, q) {
    int k, l, r;
    ll c;
    assert (cin >> k >> l >> r >> c);
    assert (k == 1 || k == 2);
    assert (1 <= l && l <= r && r <= n);
    assert (-10000 <= c && c <= 10000);
    l--; // [l,r)
    if (k == 1) { // update interval
      st.update(l, r, c);
    }
    if (k == 2) {
      cout << st.query(l, r) << endl;
    }
  }
}
