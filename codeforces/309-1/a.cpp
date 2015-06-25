#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

const ll mod = 1e9 + 7;
const int M = 1e6 + 10;
ll fact[M];

ll invmod(ll x) {
  ll e = mod - 2;
  ll sum = 1;
  ll cur = x;
  while (e > 0) {
    if (e % 2) {
      sum *= cur;
      sum %= mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

const int K = 1010;
int c[K];
int tot[K];
int main(void){
  int k;
  cin >> k;
  REP(i, 0, k) {
    cin >> c[i];
  }
  tot[k] = 0;
  REP(i, 0, k) {
    tot[i] = (i == 0 ? 0 : tot[i - 1]) + c[i];
  }
  ll tmp = 1;
  REP(i, 0, M) {
    fact[i] = tmp;
    tmp *= i + 1;
    tmp %= mod;
  }
  ll sum = fact[tot[k - 1]];
  REP(i, 0, k) {
    sum *= invmod(tot[i]);
    sum %= mod;
    sum *= invmod(fact[c[i] - 1]);
    sum %= mod;
  }
  cout << sum << endl;
}
