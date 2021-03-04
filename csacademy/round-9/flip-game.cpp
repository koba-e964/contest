#include <iostream>
#include <vector>

using namespace std;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
typedef vector<int> VI;
typedef long long ll;

int main() {
    int n, m;
    cin >> n >> m;
    vector<VI> dp(n, VI(m));
    REP(i, 0, n) {
        REP(j, 0, m) {
            cin >> dp[i][j];
        }
        if (dp[i][0] == 0) {
            REP(j, 0, m) {
                dp[i][j] = !dp[i][j];
            }
        }
    }
    ll tot = ll(n) << (m - 1);
    REP(i, 1, m) {
        int x = 0;
        REP(j, 0, n) {
            x += dp[j][i];
        }
        tot += ll(max(x, n - x)) << (m - 1 - i);
    }
    cout << tot << endl;
}
