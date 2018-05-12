#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;

const int N = 100100;
int n;
ll s[N];

// Enumerates S such that sum(S) <= limit
// returns whether ans has overfull elements 
bool dfs2(int v, ll acc, ll limit, VL &ans, int k) {
    if (acc > limit) return false;
    if ((int) ans.size() > k) {
        return true;
    }
    if (v >= n) {
        return false;
    }
    REP(i, v, n) {
        if (acc + s[i] > limit) break;
        ans.push_back(acc + s[i]);
        bool res = dfs2(i + 1, acc + s[i], limit, ans, k);
        if (res) return true;
    }
    return false;
}

int main(void) {
    int k;
    cin >> n >> k;
    REP(i, 0, n) cin >> s[i];
    ll base = 0;
    REP(i, 0, n) {
        if (s[i] < 0) {
            base += s[i];
            s[i] = -s[i];
        }
    }
    sort(s, s + n);
    ll fail = -1, pass = 21e9;
    while (pass - fail > 1) {
        ll mid = (pass + fail) / 2;
        int rem = k - 2;
        VL tmp;
        if (dfs2(0, 0, mid, tmp, rem)) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    VL ans;
    ans.push_back(0);
    dfs2(0, 0, pass - 1, ans, k);
    while ((int) ans.size() < k) ans.push_back(pass);
    sort(ans.begin(), ans.end());
    REP(i, 0, k) {
        cout << ans[i] + base << endl;
    }
}
