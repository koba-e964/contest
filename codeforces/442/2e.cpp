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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


const int SIZE = 1 << 19;

// Reference: http://mmxsrup.hatenablog.com/category/%E9%81%85%E5%BB%B6%E8%A9%95%E4%BE%A1%E3%82%BB%E3%82%B0%E3%83%A1%E3%83%B3%E3%83%88%E6%9C%A8
// with a little modification (add, min)
//(1)区間に一様加算 (2)区間の合計値
struct segtree{
public:
  //seg:区間の合計値 lazy:区間に対して、加える値でまだ遅延しているもの
  vector<int> seg, lazy;//segは欲しい情報 lazyは区間に対する一様な処理を示すもの
  segtree():seg(SIZE * 2), lazy(SIZE * 2, 1){}
  void lazy_evaluate(int k, int l, int r){//遅延情報の適用方法
    if(lazy[k] != 1){
      seg[k] *= lazy[k];
      if(r - l > 1){
	lazy[k * 2 + 1] *= lazy[k];//遅延を左の子に伝搬
	lazy[k * 2 + 2] *= lazy[k];//遅延を右の子に伝搬
      }
      lazy[k] = 1;//ノードkは伝搬完了
    }
  }
  void update(int a, int b, int k, int l, int r, int x){
    lazy_evaluate(k, l, r);
    if(r <= a || b <= l) return;
    if(a <= l && r <= b){
      lazy[k] *= x; //加える
      lazy_evaluate(k, l, r);
    } else {
      update(a, b, k * 2 + 1, l, (l + r) / 2, x);
      update(a, b, k * 2 + 2, (l + r) / 2, r, x);
      seg[k] = seg[k * 2 + 1] + seg[k * 2 + 2]; //区間の合計
    }
  }
  void update_single(int a, int k, int l, int r, int x){
    lazy_evaluate(k, l, r);
    if(r <= a || a + 1 <= l) return;
    if(a <= l && r <= a + 1){
      seg[k] = x;
    }else{
      update_single(a, k * 2 + 1, l, (l + r) / 2, x);
      update_single(a, k * 2 + 2, (l + r) / 2, r, x);
      seg[k] = seg[k * 2 + 1] + seg[k * 2 + 2]; //区間の合計
    }
  }
  int query(int a, int b, int k, int l, int r){
    lazy_evaluate(k, l, r);
    if(r <= a || b <= l) return 0;//合計に影響のないもの
    if(a <= l && r <= b) return seg[k];
    int x = query(a, b, k * 2 + 1, l, (l + r) / 2);
    int y = query(a, b, k * 2 + 2, (l + r) / 2, r);
    return x + y; //左右の合計を
  }
  //update(a,b,x) := [a,b)を全てxを加える
  void update(int a, int b, int x){update(a, b, 0, 0, SIZE, x);}
  void update_single(int a, int x){update_single(a, 0, 0, SIZE, x);}
  //query(a,b) := [a,b)に対する合計値を求める
  int query(int a, int b){return query(a, b, 0, 0, SIZE);}
};

const int N = 213456;
int p[N];
VI child[N];
int st[N], en[N];

void dfs(int v, vector<PI> &ans) {
  st[v] = ans.size();
  ans.push_back(PI(v, 1));
  REP(i, 0, child[v].size()) {
    int w = child[v][i];
    dfs(w, ans);
  }
  ans.push_back(PI(v, -1));
  en[v] = ans.size();
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  p[0] = -1;
  REP(i, 0, n - 1) {
    cin >> p[i + 1];
    p[i + 1]--;
    child[p[i + 1]].push_back(i + 1);
  }
  VI t(n);
  REP(i, 0, n) {
    cin >> t[i];
  }
  // dfs
  vector<PI> euler;
  dfs(0, euler);
  if (0) {
    cerr << "Euler:\n";
    REP(i, 0, euler.size()) {
      cerr << euler[i].first << " " << euler[i].second << endl;
    }
  }
  int m = euler.size();
  assert (m <= SIZE);
  assert (2 * n == m);
  segtree seg;
  REP(i, 0, m) {
    int idx = euler[i].first;
    seg.update_single(i, t[idx] == 1 ? -1 : 1);
  }
  int q;
  cin >> q;
  REP(i, 0, q) {
    if (0) {
      cerr << "seg:";
      REP(i, 0, m) {
	cerr << " " << seg.query(i, i + 1);
      }
      cerr << endl;
    }
    string kind;
    int v;
    cin >> kind >> v;
    v--;
    if (0) {
      cerr << "subtree " << v << ": " << st[v] << " " << en[v] << endl;
    }
    if (kind == "get") {
      int tmp = seg.query(st[v], en[v]);
      int tmp2 = (en[v] - st[v]) - tmp;
      // assert (tmp2 % 4 == 0);
      cout << tmp2 / 4 << "\n";
    } else {
      seg.update(st[v], en[v], -1);
    }
  }
}
