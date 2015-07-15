#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

int n;
ll a[1001], b[1001];

ll f(ll x) {
  ll ma = a[0] + b[0] * x;
  ll mi = ma;
  REP(i, 1, n) {
    ll w = a[i] + b[i] * x;
    ma = max(ma, w);
    mi = min(mi, w);
  }
  return ma - mi;
}


int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i] >> b[i];
  }
  ll lo = 1, hi = 1.5e9;
  while (hi - lo >= 3) {
    ll m2 = (lo + hi * 2) / 3;
    ll m1 = (lo * 2 + hi) / 3;
    if (f(m1) <= f(m2)) {
      hi = m2;
    } else {
      lo = m1;
    }
  }
  int mi = f(lo);
  int mini = lo;
  REP(i, 1, hi - lo + 1) {
    ll sub = f(lo + i);
    if (mi > sub) {
      mini = lo + i;
      mi = sub;
    }
  }
  cout << mini << endl;
}
