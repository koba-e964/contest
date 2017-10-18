#include <iostream>
#include <cassert>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;

const int B = 17;

int zeta[1 << B];
int tesla[1 << B];

int main(void) {
    ios::sync_with_stdio(false);
    cin.tie(0);
    int n;
    cin >> n;
    vector<int> a(n);
    REP(i, 0, n) {
        cin >> a[i];
        zeta[a[i]] += 1;
    }
    REP(i, 0, B) {
        REP(bits, 0, 1 << B) {
            if ((bits & 1 << i) == 0) {
                zeta[bits] += zeta[bits ^ 1 << i];
            }
        }
    }
    REP(bits, 1, 1 << B) {
        int v = bits;
        int tmp = 0;
        while (v > 0) {
            if (zeta[v] > 0) {
                tmp = v;
                break;
            }
            v = (v - 1) & bits;
        }
        tesla[bits] = tmp;
    }
    int ma = 0;
    REP(i, 0, n) {
        int t = tesla[(1 << B) - 1 - a[i]];
        ma = max(ma, a[i] | t);
    }
    ll tot = 0;
    REP(i, 0, n) {
        if ((a[i] & ma) == a[i]) {
            tot += zeta[ma - a[i]];
        }
        if (ma == a[i]) {
            tot -= 1;
        }
    }
    assert (tot % 2 == 0);
    cout << ma << " " << tot / 2 << "\n";
}
