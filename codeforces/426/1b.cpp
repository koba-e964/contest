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
const int DEBUG = 0;


const ll inf = 5e15;

// Reference: http://mmxsrup.hatenablog.com/category/%E9%81%85%E5%BB%B6%E8%A9%95%E4%BE%A1%E3%82%BB%E3%82%B0%E3%83%A1%E3%83%B3%E3%83%88%E6%9C%A8
// with a little modification (add, max)
//(1)区間に一様加算 (2)区間の合計値
struct segtree{
public:
    const int SIZE = 1 << 16;
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
        if(r <= a || b <= l) return -inf;//合計に影響のないもの
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


const int N = 40000;
ll dp[51][N];


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, k;
  cin >> n >> k;
  VI a(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  {
    set<int> dist;
    REP(i, 0, n) {
      dist.insert(a[i]);
      dp[0][i] = dist.size();
      if (DEBUG) {
	cerr << "dp[0][" << i << "]=" << dp[0][i] << endl;
      }
    }
  }
  REP(i, 1, k) {
    map<int, int> last;
    segtree st;
    REP(j, 0, n) {
      int prev = 0;
      if (last.count(a[j]) != 0) {
	prev = last[a[j]];
      }
      st.update(prev, j, 1);
      st.update(j, j + 1, dp[i - 1][j]);
      last[a[j]] = j;
      dp[i][j] = st.query(0, j);
      if (DEBUG) {
	cerr << "st:";
	REP(l, 0, n) {
	  cerr << " " << st.query(l, l + 1);
	}
	cerr << endl;
	cerr << "dp[" << i << "][" << j << "]=" << dp[i][j] << endl;
      }
    }
  }
  cout << dp[k - 1][n - 1] << "\n";
}
