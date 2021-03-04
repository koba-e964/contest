#include <iostream>
#include <vector>
#include <tuple>
#include <set>

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, PI> PLPI;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)


int main() {
    int n;
    cin >> n;
    VL a(n);
    REP(i, 0, n) cin >> a[i];
    VI p(n);
    REP(i, 0, n) {
        cin >> p[i];
        p[i]--;
    }
    VL acc(n + 1);
    REP(i, 0, n) acc[i + 1] = acc[i] + a[i];
    set<PLPI> db;
    set<PI> xy;
    db.insert(PLPI(acc[n], PI(0, n)));
    xy.insert(PI(0, n));
    REP(i, 0, n) {
        auto x = *db.rbegin();
        cout << x.first << endl;
        int l, r;
        auto it = xy.lower_bound(PI(p[i], 1e7));
        it--;
        tie(l, r) = *it;
        ll sum = acc[r] - acc[l];
        db.erase(PLPI(sum, PI(l, r)));
        xy.erase(PI(l, r));
        if (l < p[i]) {
            xy.insert(PI(l, p[i]));
            db.insert(PLPI(acc[p[i]] - acc[l], PI(l, p[i])));
        }
        if (p[i] + 1 < r) {
            xy.insert(PI(p[i] + 1, r));
            db.insert(PLPI(acc[r] - acc[p[i] + 1], PI(p[i] + 1, r)));
        }
    }
}
