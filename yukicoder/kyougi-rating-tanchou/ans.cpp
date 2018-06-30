#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

const int inf = -1000;

// Reference: http://mmxsrup.hatenablog.com/category/%E9%81%85%E5%BB%B6%E8%A9%95%E4%BE%A1%E3%82%BB%E3%82%B0%E3%83%A1%E3%83%B3%E3%83%88%E6%9C%A8
// with a little modification (add, max)
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
            seg[k] = max(seg[k * 2 + 1], seg[k * 2 + 2]); //区間の合計
        }
    }  
    ll query(int a, int b, int k, int l, int r){
        lazy_evaluate(k, l, r);
        if(r <= a || b <= l) return inf;//合計に影響のないもの
        if(a <= l && r <= b) return seg[k];
        ll x = query(a, b, k * 2 + 1, l, (l + r) / 2);
        ll y = query(a, b, k * 2 + 2, (l + r) / 2, r);
        return max(x, y); //左右の合計を
    }
    //update(a,b,x) := [a,b)を全てxを加える
    void update(int a, int b, ll x){update(a, b, 0, 0, SIZE, x);}
    //query(a,b) := [a,b)に対する合計値を求める
    ll query(int a, int b){return query(a, b, 0, 0, SIZE);}
};


int dp[200010];

int main(void) {
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i] -= i + 1;
  }
  // Coord compression
  map<ll, int> tbl;
  {
    vector<ll> inv_tbl;
    set<int> vals;
    vals.insert(0);
    REP(i, 0, n) {
      if (a[i] >= 0) {
      vals.insert(a[i]);
      }
    }
    inv_tbl = vector<ll>(vals.begin(), vals.end());
    sort(inv_tbl.begin(), inv_tbl.end());
    REP(i, 0, inv_tbl.size()) {
      tbl[inv_tbl[i]] = i;
    }
  }
  int m = tbl.size();
  segtree dp;
  REP(i, 0, n) {
    if (tbl.count(a[i])) {
      int idx = tbl[a[i]];
      int lo = idx;
      int hi = m;
      while (hi - lo > 1) {
	int mid = (hi + lo) / 2;
	ll val = dp.query(idx, idx + 1);
	if (dp.query(idx, mid + 1) == val) {
	  lo = mid;
	} else {
	  hi = mid;
	}
      }
      dp.update(idx, lo + 1, 1);
    }
  }
  cout << n - dp.query(m - 1, m) << endl;
}
