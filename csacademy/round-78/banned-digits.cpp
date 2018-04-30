#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

ll dp[20][2];
ll p10[19];
int dig[19];
ll quo[19];

int main(void) {
    VI a(10);
    REP(i, 0, 10) cin >> a[i];
    ll n;
    cin >> n;
    p10[0] = 1;
    REP(i, 1, 19) p10[i] = p10[i - 1] * 10;
    REP(i, 0, 19) {
        quo[i] = n / p10[i];
        dig[i] = quo[i] % 10;
    }
    for (int i = 18; i >= 0; --i) {
        REP(b, 0, 10) {
            if (a[b]) continue;
            if ((b > 0 || i == 0) && b <= quo[i]) {
                if (quo[i] == b) dp[i][1] += 1;
                else dp[i][0] += 1;
            }
            dp[i][0] += dp[i + 1][0] + (b < dig[i] ? dp[i + 1][1] : 0);
            if (dig[i] == b) {
                dp[i][1] += dp[i + 1][1];
            }
        }
    }
    cout << dp[0][0] << endl;
}
