#include <iomanip>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const ll mod = 998244353;

// The author read the editorial before solving this.
int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    int n;
    string s;
    cin >> n >> s;
    int dif = 0;
    ll c = 1;
    REP(i, 0, 2 * n - 1) {
        ll olddif = dif;
        if (s[i] == 'W') {
            dif++;
        } else {
            dif--;
        }
        if (dif == 0) {
            continue;
        }
        if (olddif == 0) {
            if (i != 0) {
                c = c * 2 % mod;
            }
            continue;
        }
        if ((dif > 0) == (dif - olddif > 0)) {
            continue;
        }
        ll x = abs(olddif);
        c = c * x % mod;
        c = c * (x - 1) % mod;
    }
    c = c * ((mod + 1) / 2) % mod;
    cout << c << "\n";
  }
}
