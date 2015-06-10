#include <sstream>
#include <iostream>
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const ll mod = 1e9+7;
const int N = 2e6 + 10;
ll fact[N];

ll invmod(ll v, ll m) {
  ll sum = 1, cur = v % m;
  ll e = m - 2;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % m;
    }
    cur = cur * cur % m;
    e /= 2;
  }
  return sum;
}


ll comb(ll n, ll k) {
  if (n < k || k < 0 || n < 0) {
    return 0;
  }
  ll sum = fact[n];
  sum = sum * invmod(fact[k], mod) % mod;
  sum = sum * invmod(fact[n - k], mod) % mod;
  return sum;
}

void solve(const string &s) {
  stringstream ss;
  ss << s;
  char c = ss.get();
  ss.ignore();
  ll n,k;
  ss >> n;
  ss.ignore();
  ss >> k;
  switch(c) {
  case 'C':
    cout << comb(n, k) << endl;
    break;
  case 'H':
    cout << (n == 0 ? k == 0 : comb(n + k - 1, k)) << endl;
    break;
  case 'P':
    {
      ll sum = fact[n];
      if (n >= k) {
	sum = sum * invmod(fact[n - k], mod) % mod;
      } else {
	sum = 0;
      }
      cout << sum << endl;
    }
    break;
  }
}

int main(void){
  int t;
  cin >> t;
  fact[0] = 1;
  REP(i, 1, N) {
    fact[i] = fact[i - 1] * i % mod;
  }
  REP(loop_var, 0, t) {
    string s;
    cin >> s;
    solve(s);
  }
}
