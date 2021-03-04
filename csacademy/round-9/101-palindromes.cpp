#include <iostream>
#include <string>
#include <vector>

using namespace std;
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;
const int N = 201;

ll dp[N][N][4][101];
int dig[N];

// Tags: palindrome, dp
int main() {
    int tblrem[4] = {11, 0, 92, 2};
    int n;
    string s;
    cin >> n >> s;
    REP(i, 0, n) dig[i] = s[i] - '0';
    REP(i, 0, n + 1) dp[i][i][0][0] = 1;
    REP(l, 1, n + 1) {
        REP(i, 0, n - l + 1) {
            int j = i + l;
            if (l == 1) {
                (dp[i][j][1][dig[i]] += 1) %= mod;
                (dp[i][j][0][0] += 1) %= mod;
            } else {
                REP(rem, 0, 101) {
                    REP(k, 0, 4) {
                        ll val = dp[i][j - 1][k][rem] + dp[i + 1][j][k][rem] - dp[i+1][j-1][k][rem] + mod;
                        (dp[i][j][k][rem] += val) %= mod;
                        // same
                        if (dig[i] == dig[j - 1]) {
                            int nxtrem = (10 * rem + tblrem[k] * dig[i]) % 101;
                            (dp[i][j][(k+2)%4][nxtrem] += dp[i+1][j-1][k][rem]) %= mod;
                        }
                    }
                }
            }
        }
    }
    ll tot = 0;
    REP(i, 0, 4) {
        (tot += dp[0][n][i][0]) %= mod;
    }
    cout << (tot + mod - 1) % mod << endl;
}
