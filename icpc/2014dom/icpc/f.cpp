#include <iostream>
#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

#define rep(i, n)       reps(i, 0, n)
#define reps(i, m, n)   for (int i = m; i < int(n); ++i)

const int hoge[6][4] = {
    { 1, 2, 4, 3 },
    { 0, 3, 5, 2 },
    { 0, 1, 5, 4 },
    { 0, 4, 5, 1 },
    { 0, 2, 5, 3 },
    { 1, 3, 4, 2 }
};

struct Dice {
    int t, s;
    void move(int dir) {
        int tt = t, ss = s;
        if (dir == 0) { // temae
            t = 5 - ss;
            s = tt;
        } else if (dir == 3) { // oku
            t = ss;
            s = 5 - tt;
        } else if (dir == 1) { // hidari
            int idx = -1;
            rep (i, 4) if (hoge[t][i] == s) idx = i;
            s = hoge[t][(idx + 1) % 4];
        } else if (dir == 2) { // migi
            int idx = -1;
            rep (i, 4) if (hoge[t][i] == s) idx = i;
            s = hoge[t][(idx + 3) % 4];
        }
    }
};

Dice d;
int s[6];
char tmp[30303];
const char c[4] = { 'E', 'N', 'S', 'W' };

bool solve(int idx)
{
    int bt = d.t, bs = d.s;
    if (s[d.s] == 0) {
        tmp[idx - 1] = '\0';
        rep (i, 6) if (s[i] != 0) return false;
        return true;
    }
    int c = 0;
    rep (i, 6) c += s[i];
    if (s[bs] + s[5 - bs] > 2 * c) return false;
    if (s[0] + s[5] > s[1] + s[2] + s[3] + s[4] + 1) return false;
    if (s[1] + s[4] > )
    --s[d.s];
    pair<int, int> best[4];
    rep (i, 4) {
        d.move(i);
        best[i].first  = s[d.s];
        best[i].second = 3 - i;
        d.move(3 - i);
    }
    sort(best, best + 4, greater<pair<int, int> >());
    rep (i, 4) {
        d.t = bt; d.s = bs;
        d.move(3 - best[i].second);
        tmp[idx] = c[3 - best[i].second];
        if (solve(idx + 1)) {
            //rep (k, 6) cout << s[i] << " ";
            //cout << endl;
            return true;
        }
    }
    ++s[bs];
    d.t = bt; d.s = bs;
    return false;
}

int main()
{
    int t[6], p, q, tt[6];
    while (true) {
        string ans;
        rep (i, 6) cin >> t[i];
        if (t[5] == 0) break;
        cin >> p >> q;
        do {
            d.t = 1; d.s = 0;
            rep (i, 6) s[i] = t[i];
            tmp[0] = 'E';
            bool ok = solve(1);
            string res(tmp);
            if (ok && (ans.empty() || res < ans)) {
                ans = res;
                rep (i, 6) tt[i] = t[i];
            }
        } while (next_permutation(t, t + 6));
        if (ans.empty())
            cout << "impossible" << endl;
        else {
            cout << ans << endl;
            rep (i, 6) cout << tt[i] << " ";
            cout << endl;
        }
    }
}

