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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

ll gcd(ll x,ll y){
  while(y!=0){
    ll r=x%y;
    x=y;y=r;
  }return x;
}

struct frac{
  ll x,y;frac(ll x,ll y):x(x),y(y){
    reduce();
  }
  frac operator+(const frac& f)const{
    return frac(x*f.y+f.x*y, y*f.y);
  }
  frac operator*(const frac& f)const{
    return frac(x*f.x,y*f.y);
  }
  bool operator<(const frac &f) const {
    return x*f.y < y*f.x;
  }
  bool operator==(const frac &f) const {
    return x*f.y == y*f.x;
  }
  void reduce(){
    ll g=gcd(x,y);
    x/=g;
    y/=g;
    if(y<0){
      x=-x;y=-y;
    }
  }
};

// https://en.wikipedia.org/wiki/Continued_fraction#Best_rational_within_an_interval
vector<VL> cont(ll x, ll y, VL &path) {
  if (y == 0) {
    return vector<VL>(1, path);
  }
  ll q = x / y;
  ll r = x % y;
  path.push_back(q);
  vector<VL> res = cont(y, r, path);
  if (r == 0 && q >= 1) {
    VL cp(path);
    cp[path.size() - 1] = q - 1;
    cp.push_back(1);
    res.push_back(cp);
  }
  path.pop_back();
  return res;
}

PL expand(const VL& a) {
  int n = a.size();
  ll x = 1, y = 0;
  for (int i = n - 1; i >= 0; --i) {
    ll nx = y + x * a[i];
    y = x;
    x = nx;
  }
  return PL(x, y);
}

PL find(frac lo, frac hi) {
  VL tmp;
  vector<VL> fst = cont(lo.x, lo.y, tmp);
  tmp.clear();
  vector<VL> snd = cont(hi.x, hi.y, tmp);
  if (DEBUG) {
    cerr << "fst:" << endl;
    REP(i, 0, fst.size()) {
      REP(j, 0, fst[i].size()) cerr << " " << fst[i][j];
      cerr << endl;
    }
    cerr << "snd:" << endl;
    REP(i, 0, snd.size()) {
      REP(j, 0, snd[i].size()) cerr << " " << snd[i][j];
      cerr << endl;
    }
  }
  PL inf(1e18, 1e18);
  PL ans = inf;
  REP(i, 0, fst.size()) {
    REP(j, 0, snd.size()) {
      VL &p = fst[i];
      VL &q = snd[j];
      int len = 0;
      while (len < (int) min(p.size(), q.size())) {
        if (p[len] == q[len]) len++;
        else break;
      }
      VL res(len + 1);
      REP(k, 0, len) res[k] = p[k];
      if (len == (int) p.size()) {
        res[len] = q[len] + 1;
      } else if (len == (int) q.size()) {
        res[len] = p[len] + 1;
      } else {
        res[len] = min(p[len], q[len]) + 1;
      }
      if (DEBUG) {
        cerr << "p:";
        for(int e: p)cerr<<" "<<e;
        cerr << endl;
        cerr << "q:";
        for(int e: q)cerr<<" "<<e;
        cerr << endl;
        cerr << "res:";
        for(int e: res)cerr<<" "<<e;
        cerr << endl;
      }
      PL f = expand(res);
      if (f != PL(lo.x, lo.y) && f != PL(hi.x, hi.y)) {
        ans = min(ans, f);
      }
    }
  }
  return ans >= inf ? PL(-1, -1) : ans;
}

PL solve(const vector<PL> &a) {
  int n = a.size();
  frac mi(0, 1), ma(1, 0);
  REP(i, 0, n - 1) {
    ll dx = a[i].first - a[i + 1].first;
    ll dy = a[i].second - a[i + 1].second;
    if (dx == 0) {
      if (dy >= 0) return PL(-1, -1);
    } else if (dx > 0) {
      ma = min(ma, frac(-dy, dx));
    } else {
      mi = max(mi, frac(-dy, dx));
    }
  }
  if (DEBUG) {
    cerr << "ma = " << ma.x << "/" << ma.y << endl;
    cerr << "mi = " << mi.x << "/" << mi.y << endl;
  }
  if (!(mi < ma)) return PL(-1, -1);
  return find(mi, ma);
}

#if 1
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(case_nr, 1, t + 1) {
    int n;
    cin >> n;
    vector<PL> a(n);
    REP(i, 0, n) {
      ll c, j;
      cin >> c >> j;
      a[i] = PL(c, j);
    }
    PL res = solve(a);
    cout << "Case #" << case_nr << ": ";
    if (res.first < 0) {
      cout << "IMPOSSIBLE\n";
    } else {
      cout << res.first << " " << res.second << "\n";
    }
  }
}
#else
int main(void) {
  ll x, y, z, w;
  cin >> x >> y >> z >> w;
  frac lo(x, y);
  frac hi(z, w);
  PL ans = find(lo, hi);
  cerr << ans.first << "/" << ans.second << endl;
}
#endif
