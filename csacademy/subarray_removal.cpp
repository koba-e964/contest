#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

const ll inf = 5e15;

int main(void) {
    int n;
    cin >> n;
    VL a(n);
    REP(i, 0, n) {
        cin >> a[i];
    }
    VL b(n + 1);
    b[0] = 0;
    REP(i, 0, n) {
        b[i + 1] = b[i] + a[i];
    }
    VL left(n + 1, -inf);
    ll mi = 0;
    REP(i, 1, n + 1) {
        mi = min(mi, b[i - 1]);
        left[i] = max(left[i - 1], b[i] - mi);
    }
    VL right(n + 1, -inf);
    ll ma = b[n];
    for (int i = n - 1; i >= 0; --i) {
        ma = max(ma, b[i + 1]);
        right[i] = max(right[i + 1], ma - b[i]);
    }
    ma = -5e15;
    REP(i, 0, n) {
        ma = max(ma, left[i] + right[i + 1]);
    }
    REP(i, 1, n) {
        ma = max(ma, left[i]);
    }
    REP(i, 1, n) {
        ma = max(ma, b[n] - b[i]);
    }
    cout << ma << endl;
}
