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
const ll inf = 5e15;

// Reference: http://mmxsrup.hatenablog.com/category/%E9%81%85%E5%BB%B6%E8%A9%95%E4%BE%A1%E3%82%BB%E3%82%B0%E3%83%A1%E3%83%B3%E3%83%88%E6%9C%A8
// with a little modification (add, min)
//(1)区間に一様加算 (2)区間の合計値
struct segtree{
public:
    const int SIZE = 1 << 19;
    //seg:区間の合計値 lazy:区間に対して、加える値でまだ遅延しているもの
    vector<ll> seg, lazy;//segは欲しい情報 lazyは区間に対する一様な処理を示すもの
  segtree():seg(SIZE * 2, inf), lazy(SIZE * 2){}
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


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, q;
  cin >> n >> q;
  int a, b;
  cin >> a >> b;
  segtree st1, st2;
  // st1: dp1[u] + u, dp2[u] + u, st2: dp1[u] - u, dp2[u] - u
  st1.update(1, 2 * n + 1, -inf + inf / 2);
  st2.update(1, 2 * n + 1, -inf + inf / 2);
  st1.update(a, a + 1, a - inf / 2);
  st2.update(a, a + 1, -a - inf / 2);
  VL x(q + 2);
  x[0] = a;
  x[1] = b;
  REP(i, 2, q + 2) {
    cin >> x[i];
    ll newval1 = st2.query(1, x[i]) + x[i];
    newval1 = min(newval1, st1.query(x[i], n + 1) - x[i]);
    ll newval2 = st2.query(n + 1, n + x[i]) + x[i];
    newval2 = min(newval2, st1.query(n + x[i], 2 * n + 1) - x[i]);
    st1.update(1, 2 * n + 1, abs(x[i] - x[i - 1]));
    st2.update(1, 2 * n + 1, abs(x[i] - x[i - 1]));
#define UPMAX(st, idx, v) do { ll oldv = st.query(idx, (idx) + 1); \
      if (oldv > (v)) { st.update(idx, (idx) + 1, (v) - oldv);}} while (0)
    UPMAX(st1, x[i - 1], newval2 + x[i - 1]);
    UPMAX(st2, x[i - 1], newval2 - x[i - 1]);
    UPMAX(st1, n + x[i - 1], newval1 + x[i - 1]);
    UPMAX(st2, n + x[i - 1], newval1 - x[i - 1]);
    if (0) {
      cerr << "i = " << i << "\n";
      cerr << "st1:";
      REP(i, 1, 2 * n + 1) {
	cerr << " " << st1.query(i, i + 1);
      }
      cerr << "\n";
      cerr << "st2:";
      REP(i, 1, 2 * n + 1) {
	cerr << " " << st2.query(i, i + 1);
      }
      cerr << "\n";
    }
  }
  ll mi = inf;
  REP(i, 1, n + 1) {
    mi = min(mi, st1.query(i, i + 1) - i);
    mi = min(mi, st1.query(n + i, n + i + 1) - i);
  }
  cout << mi << "\n";
}
