#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, int> PLI;

ll sqdist(ll a, ll b, ll c, ll d) {
    return (a - c) * (a - c) + (b - d) * (b - d);
}


int main(void) {
    ios::sync_with_stdio(false);
    cin.tie(0);
    int t;
    cin >> t;
    while (t--) {
        int n;
        cin >> n;
        VL x(n), y(n);
        REP(i, 0, n) {
            cin >> x[i] >> y[i];
        }
        vector<bool> dec(n);
        dec[0] = true;
        vector<PLI> dist(n);
        REP(i, 0, n) {
            dist[i] = PLI(sqdist(x[0], y[0], x[i], y[i]), 0);
        }
        int rem = n - 1;
        vector<PI> edges;
        while (rem > 0) {
            PLI mi(1e16, -1);
            REP(i, 0, n) {
                if (dec[i]) continue;
                mi = min(mi, PLI(dist[i].first, i));
            }
            int idx = mi.second;
            assert (idx >= 0);
            // contract
            int from = dist[idx].second;
            edges.push_back(PI(from, idx));
            REP(i, 0, n) {
                if (dec[i]) continue;
                PLI ent(sqdist(x[idx], y[idx], x[i], y[i]), idx);
                dist[i] = min(dist[i], ent);
            }
            dec[idx] = true;
            rem--;
        }
        assert ((int) edges.size() == n - 1);
        VI deg(n);
        vector<VI> g(n);
        REP(i, 0, n - 1) {
            int a = edges[i].first;
            int b = edges[i].second;
            g[a].push_back(b);
            g[b].push_back(a);
            deg[a]++;
            deg[b]++;
        }
        priority_queue<PI, vector<PI>, greater<PI> > retro;
        REP(i, 0, n) {
            if (deg[i] == 1) {
                int dest = g[i][0];
                int d = min(abs(dest - i), n - abs(dest - i));
                retro.push(PI(d, i));
            }
        }
        vector<PI> elim;
        vector<bool> used(n);
        while (not retro.empty()) {
            PI dv = retro.top(); retro.pop();
            int d = dv.first;
            int v = dv.second;
            if (deg[v] < 1) continue;
            int idx = -1;
            REP(i, 0, g[v].size()) {
                int w = g[v][i];
                if (used[w]) continue;
                idx = w;
                break;
            }
            elim.push_back(PI(v, idx));
            deg[idx]--;
            used[v] = true;
            if (deg[idx] == 1) {
                int to = -1;
                REP(i, 0, g[idx].size()) {
                    int w = g[idx][i];
                    if (used[w]) continue;
                    to = w;
                    break;
                }
                int d = min(abs(idx - to), n - abs(idx - to));
                retro.push(PI(d, idx));
            }
        }
        assert ((int) elim.size() == n - 1);
        REP(i, 0, elim.size()) {
            cout << elim[i].first + 1 << " " << elim[i].second + 1 << endl;
        }
    }
}
