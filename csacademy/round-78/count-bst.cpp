#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

const ll mod = 1e9 + 7;
ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}
void add(ll &x, ll y) {
  x = (x + y) % mod;
}

const int N = 3000100;

ll cat[N];

void init(void) {
    ll cur = 1;
    cat[0] = 1;
    REP(i, 0, N - 1) {
        ll fac = (mod - 6) * powmod(i + 2, mod - 2) % mod;
        fac = (fac + 4) % mod;
        cur = cur * fac % mod;
        cat[i + 1] = cur;
    }
}

int main(void) {
    init();
    int t;
    cin >> t;
    while (t--) {
        int n, k;
        cin >> n >> k;
        if (k == 0) {
            cout << cat[n] << endl;
            continue;
        }
        VI a(k);
        int mi = n + 1, ma = 0;
        REP(i, 0, k) {
            cin >> a[i];
            ma = max(ma, a[i]);
            mi = min(mi, a[i]);
        }
        ll prod = cat[n - (ma - mi)];
        VI srt(a);
        sort(srt.begin(), srt.end());
        REP(i, 0, k - 1) {
            prod = prod * cat[srt[i + 1] - srt[i] - 1] % mod;
        }
        ll tot = 0;
        VI lft(k);
        int lo = a[0], hi = a[0]; // (lo, hi), open interval
        lft[0] = 1;
        REP(i, 1, k) {
            if (lo < a[i] && a[i] < hi) {
                break;
            }
            if (a[i] < lo) {
                lo = a[i];
            } else {
                hi = a[i];
            }
            lft[i] = 1;
        }
        VI rgt(k);
        lo = a[k - 1], hi = a[k - 1];
        rgt[k - 1] = 1;
        for (int i = k - 2; i >= 0; --i) {
            if (lo < a[i] && a[i] < hi) {
                break;
            }
            if (a[i] < lo) {
                lo = a[i];
            } else {
                hi = a[i];
            }
            rgt[i] = 1;
        }
        REP(i, 0, k) {
            // Assume root = a[i]
            if (i != 0 && i != k - 1
                && (ll)(a[i] - a[i - 1]) * (ll)(a[i] - a[i + 1]) > 0)
                continue;
            if (lft[i] && rgt[i])
                add(tot, 1);
        }
        cout << tot * prod % mod << endl;
    }
}
