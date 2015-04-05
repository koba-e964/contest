#include <iostream>
#include <iomanip>
#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;

#define rep(i, n)       reps(i, 0, n)
#define reps(i, m, n)   for (int i = m; i < int(n); ++i)

int r, n;
int h[40];

double sq(double x) { return x * x; }

bool ok(double x)
{
    rep (i, r) {
        if (sq(max(h[20 + i] - x + r, 0.0)) < sq(r) - sq(i)) return false;
        if (sq(max(h[19 - i] - x + r, 0.0)) < sq(r) - sq(i)) return false;
    }
    return true;
}

int main()
{
    while (cin >> r >> n && (r || n)) {
        int xl, xr, y;
        fill(h, h + 40, 0);
        rep (i, n) {
            cin >> xl >> xr >> y;
            reps (j, xl + 20, xr + 20) h[j] = max(h[j], y);
        }
        double al = 0, ah = 20;
        rep (i, 100) {
            double mid = (al + ah) / 2;
            (ok(mid) ? al : ah) = mid;
        }
        cout << fixed << setprecision(10) << ah << endl;
    }
}

