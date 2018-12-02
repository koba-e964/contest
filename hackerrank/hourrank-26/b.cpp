#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  vector<PL> xp(n);
  REP(i, 0, n) {
    cin >> xp[i].second;
  }
  REP(i, 0, n) {
    cin >> xp[i].first;
  }
  sort(xp.begin(), xp.end());
  VI imos(n + 1);
  int m;
  cin >> m;
  vector<PL> lr(m);
  REP(i, 0, m) cin >> lr[i].first;
  REP(i, 0, m) cin >> lr[i].second;
  REP(i, 0, m) {
    ll l = lr[i].first - lr[i].second;
    ll r = lr[i].first + lr[i].second;
    lr[i] = PL(l, r);
    int idx1 = lower_bound(xp.begin(), xp.end(), PL(l, -1)) - xp.begin();
    int idx2 = lower_bound(xp.begin(), xp.end(), PL(r, 1e18)) - xp.begin();
    imos[idx1]++;
    imos[idx2]--;
  }
  REP(i, 0, n) {
    imos[i + 1] += imos[i];
  }
  assert (imos[n] == 0);
  vector<PL> yuki;
  ll base = 0;
  REP(i, 0, n) {
    if (imos[i] == 1) {
      yuki.push_back(xp[i]);
    }
    if (imos[i] == 0) {
      base += xp[i].second;
    }
  }
  int l = yuki.size();
  VL ei(l + 1);
  REP(i, 0, l) {
    ei[i + 1] = ei[i] + yuki[i].second;
  }
  ll ma = 0;
  REP(i, 0, m) {
    ll l = lr[i].first;
    ll r = lr[i].second;
    int idx1 = lower_bound(yuki.begin(), yuki.end(), PL(l, -1)) - yuki.begin();
    int idx2 = lower_bound(yuki.begin(), yuki.end(), PL(r, 1e18)) - yuki.begin();
    ma = max(ma, ei[idx2] - ei[idx1]);
  }
  cout << base + ma << endl;
}

