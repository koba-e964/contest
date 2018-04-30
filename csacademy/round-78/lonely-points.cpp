#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

int main(void) {
    int n;
    cin >> n;
    VI a(n);
    REP(i, 0, n) cin >> a[i];
    VI intv(n - 1);
    REP(i, 1, n - 2) intv[i] = a[i + 1] - a[i];
    intv[n - 2] = min(a[1] - a[0], a[n - 1] - a[n - 2]);
    sort(intv.rbegin(), intv.rend());
    cout << max((intv[0] + 1) / 2, intv[1]) << endl;
}
