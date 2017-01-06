#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
const ll mod = 1e9 + 7;

const int W = 10000;
ll fact[W];
// \sum_{S, |S| = v} \prod{i in S} kind[i]
vector<ll> comb(const VI &kind) {
  int n = kind.size();
  VL ret(n + 1);
  ret[0] = 1;
  REP(b, 0, n) {
    for (int i = n - 1; i >= 0; --i) {
      ret[i + 1] += ret[i] * kind[b];
      ret[i + 1] %= mod;
    }
  }
  return ret;
}

int main(void){
  int n;
  cin >> n;
  VI a(n);
  VI kind(n);
  REP(i, 0, n) {
    cin >> a[i];
    if (a[i] < n) {
      kind[a[i]]++;
    }
  }
  fact[0] = 1;
  REP(i, 1, W) {
    fact[i] = fact[i - 1] * i % mod;
  }
  ll sum = 0;
  VL cb = comb(kind);
  REP(i, 0, n + 1) {
    ll tmp = fact[n - i];
    if (i % 2 == 1) {
      tmp = (mod - tmp) % mod;
    }
    tmp *= cb[i];
    sum += tmp;
    sum %= mod;
  }
  cout << sum << endl;
}
