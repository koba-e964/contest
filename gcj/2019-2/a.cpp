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
typedef pair<ll, ll> PL;
const ll mod = 1e9 + 7;
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


ll solve(const vector<PL> &a) {
  int n = a.size();
  set<frac> cand;
  REP(i, 0, n) {
    REP(j, 0, i) {
      ll dx = a[i].first - a[j].first;
      ll dy = a[i].second - a[j].second;
      if (dx * dy >= 0) continue;
      cand.insert(frac(dx, -dy));
    }
  }
  return cand.size() + 1;
}

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
    cout << "Case #" << case_nr << ": " << solve(a) << "\n";
  }
}
