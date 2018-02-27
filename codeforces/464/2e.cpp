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

//http://d.hatena.ne.jp/sune2/20140310/1394440369

struct CHT {
  typedef double T;
  typedef pair<T,T> pii;
  vector<pii> deq;              // first * x + second
  int s,t;
  CHT(int n) {                  // n : クエリ数
    deq.resize(n);
    s=0, t=0;
  }
  void add(T a, T b) {      // a : 単調減少
    const pii p(a,b);
    while(s+1<t && check(deq[t-2],deq[t-1],p)) t--;
    deq[t++] = p;
  }
  T incl_query(T x) {            // x : 単調増加
    while(s+1<t && f(deq[s], x) >= f(deq[s+1], x)) s++;
    return f(deq[s], x);
  }
  T query(T x) {           // 条件なし
    int low = s-1, high = t-1;
    while(low+1<high) {
      int mid = low+high>>1;
      if (isright(deq[mid], deq[mid+1], x)) low = mid;
      else high = mid;
    }
    return f(deq[high], x);
  }
private:
  bool isright(const pii &p1, const pii &p2, T x) {
    return (p1.second-p2.second) >= x * (p2.first-p1.first);
  }
  bool check(const pii &p1, const pii &p2, const pii &p3) {
    return (p2.first-p1.first)*(p3.second-p2.second) >=
      (p2.second-p1.second)*(p3.first-p2.first);
  }
  T f(const pii &p, T x) {
    return p.first * x + p.second;
  }
};

int main(void) {
  int q;
  scanf("%d", &q);
  ll ma = 0;
  ll sum = 0;
  int cnt = 0;
  VL acc;
  CHT cht(q);
  REP(i, 0, q) {
    int kind;
    scanf("%d", &kind);
    if (kind == 1) {
      ll x;
      scanf("%lld", &x);
      ma = x;
      acc.push_back(x);
      sum += x;
      cnt += 1;
      cht.add(1.0 / cnt, (double)(sum - ma) / (double) cnt);
    } else {
      double mean = cht.query(ma);
      printf("%.15f\n", ma - mean);
    }
  }
}
