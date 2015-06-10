#include <cassert>
#include <iostream>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int N = 10010;

int a[N];

int main(void){
  ll n,m;
  cin >> n >> m;
  n /= 1000;
  n %= m;
  // C(m, n)
  REP(i, 0, n) {
    a[i] = m - i;
  }
  REP(i, 2, n + 1) {
    int v = i;
    REP(j, 0, n) {
      int g = __gcd(a[j], v);
      a[j] /= g;
      v /= g;
      if (v == 1) {
	break;
      }
    }
    if (v != 1) {
      assert(! "not found");
    }
  }
  ll sum = 1;
  const ll mod = 1e9;
  REP(i, 0, n) {
    sum *= a[i];
    sum %= mod;
  }
  cout << sum << endl;
}
