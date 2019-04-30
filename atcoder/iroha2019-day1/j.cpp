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
const ll mod = 1e9 + 7;

/*
 * Dependencies: typedef long long ll
 * Headers: iostream
 * Verified by: ARC099-F
 *              https://beta.atcoder.jp/contests/arc099/submissions/2743855
 */

template<ll mod = (ll)1e9 + 7>
struct ModInt {
  ll x;
  ModInt(void): x(0) {}
  ModInt(ll x): x(x % mod){}
  ModInt(const ModInt &x): x(x.x) {}
  ModInt operator+(ModInt o) const {
    ll y = x + o.x;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator-(ModInt o) const {
    ll y = x - o.x + mod;
    if (y >= mod) y -= mod;
    return ModInt(y);
  }
  ModInt operator*(ModInt o) const {
    return ModInt((x * o.x) % mod);
  }
  void operator+=(ModInt o) { *this = *this + o; }
  void operator-=(ModInt o) { *this = *this - o; }
  void operator*=(ModInt o) { *this = *this * o; }
  ModInt operator-(void) const { return ModInt() - *this; }
  ll to_ll() const {
    return x;
  }
  bool operator<(ModInt o) const {
    return x < o.x;
  }
  ModInt pow(ll e) {
    assert (e >= 0);
    ModInt sum = 1;
    ModInt cur = *this;
    while (e > 0) {
      if (e % 2) {
        sum = sum * cur;
      }
      cur = cur * cur;
      e /= 2;
    }
    return sum;
  }
  ModInt inv(void) {
    return pow(mod - 2);
  }
};

template<ll mod>
ostream &operator<<(ostream &os, ModInt<mod> mi) {
  return os << mi.x;
}

const int N=200000;
ModInt<> fac[N],invfac[N];
void init(void) {
  fac[0]=1;
  REP(i, 1,N)fac[i]=fac[i-1]*i;
  REP(i, 0, N)invfac[i]=fac[i].inv();
}


ll sq(ll x) {
  ll v = sqrt(x);
  for (ll y=max(0ll,v-9);y<=v+9;++y){
    if(y*y==x)return y;
  }
  return -1;
}

ModInt<> solve(ll n,ll k){
  if (n%2==0){
    n/=2;
    if(k%2)return 0;
    k/=2;
    //decide a,b
    ll u=sq(n*n-4*k);
    if(u<0)return 0;
    ll a=(n+u)/2,b=(n-u)/2;
    return fac[n]*invfac[a]*invfac[b]*(a==b?1:2);
  }
  n/=2;
  ModInt<>tot;
  //0 a(2b+1)=k,a+b=n
  ll u=sq((2*n-1)*(2*n-1)+8*(n-k));
  if(u>=0){
    ll a=(2*n-1-u)/2,b;
    if(a>=0&&a%2==0){
      a/=2;
      b=n-a;
      tot+=fac[n]*invfac[a]*invfac[b];
    }
    a=(2*n-1+u)/2;
    //a--,b++;
    if(a>=0&&a%2==0&&u!=0) {
      a/=2;
      b=n-a;
      tot+=fac[n]*invfac[a]*invfac[b];
    }
  }
  return tot*2;
  
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  int q;
  cin >> q;
  while (q--) {
    ll n, k;
    cin >> n >> k;
    cout<<solve(n, k) << endl;
  }
}
