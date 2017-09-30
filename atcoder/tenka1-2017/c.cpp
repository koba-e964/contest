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
const ll mod = 1e9 + 7;

ll gcd(ll x,ll y){
  while(y!=0){
    ll r=x%y;
    x=y;y=r;
  }
  return x;
}

struct frac{
  ll x,y;
  frac(ll x,ll y):x(x),y(y){
    reduce();
  }
  frac operator+(const frac& f)const{
    return frac(x*(f.y)+f.x*(y),y*f.y);
  }
  frac operator-(const frac& f)const{
    return frac(x*(f.y)-f.x*(y),y*f.y);
  }
  frac operator*(const frac& f)const{
    return frac(x*f.x,y*f.y);
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


int main(void) {
  ll n;
  cin >> n;
  frac t(4, n);
  REP(i, 1, 3501) {
    if (4 * i <= n) { continue; }
    REP(j, 1, 3501) {
      if (4.0 / n - 1.0 / i - 1.0 / j <= 0) { continue; }
      frac q = t - frac(1, i) - frac(1, j);
      if (q.x == 1 && q.y >= 1) {
	cout << i << " " << j << " " << q.y << endl;
	return 0;
      }
    }
  }
  assert (0);
}
