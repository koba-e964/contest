#include <iostream>
#include <set>
#include <map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;

int main(void){
    int n;
    cin >> n;
    vector<set<string>> langs(n);
    REP(i, 0, n) {
        int k;
        cin >> k;
        set<string> xs;
        REP(_, 0, k) {
            string tmp;
            cin >> tmp;
            xs.insert(tmp);
        }
        langs[i] = xs;
    }
    vector<vector<int>> g(n, vector<int>(n, 0));
    REP(i, 0, n) {
        REP(j, i + 1, n) {
            bool ok = false;
            for (auto x: langs[i]) {
                if (langs[j].count(x)) {
                    ok = true;
                    break;
                }
            }
            if (ok) g[i][j] = g[j][i] = 1;
        }
    }
    vector<int> deg(n);
    REP(i, 0, n) REP(j, 0, n) deg[i] += g[i][j];
    vector<ll> path2(n);
    REP(i, 0, n) {
        REP(j, 0, n) {
            if (g[i][j]) path2[i] += deg[j] - 1;
        }
    }
    // 包除原理の要領で、
    // v1 = v4 (-1), v1 = v5 (-1), v2 = v4 (-1), v2 = v5 (-1),
    // v1 = v4 and v2 = v5 (+1), v1 = v5 and v2 = v4 (+1)
    // のパターンを数える
    ll t0 = 0;
    ll t1 = 0;
    ll t2 = 0;
    ll t3 = 0;
    ll t5 = 0;
    ll t6 = 0;
    REP(i, 0, n) {
        t0 += path2[i] * path2[i];
        REP(j, 0, n) {
            if (g[i][j]) t3 += (deg[j] - 1) * (deg[j] - 1);
        }
        t6 += path2[i];
    }
    REP(i, 0, n) REP(j, 0, n) if (g[i][j]) {
        REP(k, 0, n) {
            if (g[i][k] && g[j][k]) {
                t1 += deg[k] - 1;
                t5 += 1;
            }
        }
    }
    REP(i, 0, n) REP(j, i + 1, n) {
        ll s = 0;
        REP(k, 0, n) s += g[i][k] && g[j][k];
        t2 += s * s * 2;
    }
    cout << t0 - t1 * 2 - t2 - t3 + t5 + t6 << "\n";
}
