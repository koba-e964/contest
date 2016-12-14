#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;
const ll mod = 1e9 + 7;

ll fact[100];

int main(void){
  int n, k;
  cin >> n >> k;
  VL a(k);
  REP(i, 0, k) { cin >> a[i]; }
  a.push_back(0);
  a.push_back((1LL << n) - 1);
  sort(a.begin(), a.end());
  REP(i, 0, k + 1) {
    if ((a[i] & a[i + 1]) != a[i]) {
      cout << 0 << endl;
      return 0;
    }
  }
  fact[0] = 1;
  REP(i, 1, 100) {
    fact[i] = fact[i - 1] * i % mod;
  }
  ll tmp = 1;
  REP(i, 0, k + 1) {
    int v = __builtin_popcountll(a[i] ^ a[i + 1]);
    tmp = tmp * fact[v] % mod;
  }
  cout << tmp << endl;
}
