#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

int main(void){
  int n;
  ll k;
  cin >> n >> k;
  string s;
  cin >> s;
  vector<ll> acc(n + 1);
  ll now = 0;
  REP(i, 0, n) {
    int d = s[i] - '0';
    acc[i + 1] = now + 1;
    now += 1 - d;
  }
  ll ans = 0;
  REP(i, 1, min((ll) n, k) + 1) {
    ll q = (k - i) / n;
    ans = max(ans, acc[i] + q * max(0LL, now));
  }
  cout << ans << endl;
}
