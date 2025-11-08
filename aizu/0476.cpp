#include <algorithm>
#include <iostream>
#include <queue>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

int dp[2][51][301][301];
const int INF = 1 << 28;

int main(void){
    int n, c;
    cin >> n >> c;
    int ma, ms, mw;
    cin >> ma >> ms >> mw;
    int total = ma + ms + mw;
    vector<int> a(n), s(n), w(n);
    REP(i, 0, n) {
        cin >> a.at(i) >> s.at(i) >> w.at(i);
    }
    REP(i, 0, 2) REP(ii, 0, n + 1) REP(j, 0, 301) REP(k, 0, 301) dp[i][ii][j][k] = INF;
    dp[0][0][ma][ms] = 0;
    REP(i, 0, n) {
        REP(ii, 0, n + 1) REP(j, 0, 301) REP(k, 0, 301) dp[1][ii][j][k] = INF;
        REP(ii, 0, i + 1) {
            REP(x0, 0, total + 1) {
                REP(x1, 0, total - x0 + 1) {
                    if (dp[0][ii][x0][x1] > c) continue;
                    dp[1][ii][x0][x1] = dp[0][ii][x0][x1];
                }
            }
            // (+1, 0) (-1, 0)
            REP(x0, 0, total) {
                REP(x1, 0, total - x0 + 1) {
                    dp[1][ii][x0 + 1][x1] = min(dp[1][ii][x0 + 1][x1], dp[1][ii][x0][x1] + 1);
                }
            }
            for (int x0 = total; x0 >= 1; x0--) {
                REP(x1, 0, total - x0 + 1) {
                    dp[1][ii][x0 - 1][x1] = min(dp[1][ii][x0 - 1][x1], dp[1][ii][x0][x1] + 1);
                }
            }
            // (0, +1) (0, -1)
            REP(x0, 0, total + 1) {
                REP(x1, 0, total - x0) {
                    dp[1][ii][x0][x1 + 1] = min(dp[1][ii][x0][x1 + 1], dp[1][ii][x0][x1] + 1);
                }
            }
            REP(x0, 0, total + 1) {
                for (int x1 = total - x0; x1 >= 1; x1--) {
                    dp[1][ii][x0][x1 - 1] = min(dp[1][ii][x0][x1 - 1], dp[1][ii][x0][x1] + 1);
                }
            }
            // (+1, -1) (-1, +1)
            REP(x0, 0, total + 1) {
                REP(x1, 0, total - x0 + 1) {
                    dp[1][ii][x0 + 1][x1 - 1] = min(dp[1][ii][x0 + 1][x1 - 1], dp[1][ii][x0][x1] + 1);
                }
            }
            for (int x0 = total; x0 >= 1; x0--) {
                REP(x1, 0, total - x0 + 1) {
                    dp[1][ii][x0 - 1][x1 + 1] = min(dp[1][ii][x0 - 1][x1 + 1], dp[1][ii][x0][x1] + 1);
                }
            }
        }
        for (int ii = i; ii >= 0; ii--) {
            REP(x0, 0, total + 1) REP(x1, 0, total - x0 + 1) {
                int x2 = total - x0 - x1;
                int win = (x0 > a.at(i)) + (x1 > s.at(i)) + (x2 > w.at(i)) >= 2;
                if (win) {
                    dp[1][ii + 1][x0][x1] = min(dp[1][ii + 1][x0][x1], dp[1][ii][x0][x1]);
                }
            }
        }
        swap(dp[0], dp[1]);
    }
    int ans = 0;
    REP(ii, 0, n + 1) REP(x0, 0, total + 1) REP(x1, 0, total - x0 + 1) {
        if (dp[0][ii][x0][x1] <= c) {
            ans = max(ans, ii);
        }
    }
    cout << ans << "\n";
    exit(0);
}
