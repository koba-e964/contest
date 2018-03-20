#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;

const ll inf = 3e18;

void add(ll &x, ll y) {
  x += y;
  if (x >= inf) x = inf;
}

int main(void) {
  string s;
  ll k;
  cin >> s >> k;
  int n = s.length();
  vector<VI> nxt(n + 1, VI(26, -1));
  for (int i = n - 1; i >= 0; --i) {
    nxt[i] = nxt[i + 1];
    nxt[i][s[i] - 'a'] = i;
  }
  VL dpc(n + 1); // cumulative
  dpc[n] = 1;
  for (int i = n - 1; i >= 0; --i) {
    ll tmp = 1;
    REP(j, 0, 26) {
      if (nxt[i][j] >= 0) {
	add(tmp, dpc[nxt[i][j] + 1]);
      }
    }
    dpc[i] = tmp;
  }
  if (k >= dpc[0]) {
    cout << "Eel" << endl;
    return 0;
  }
  string ans;
  int pos = 0;
  while (k > 0) {
    ll tmp = 1;
    int idx = -1;
    REP(i, 0, 26) {
      int ni = nxt[pos + 1][i];
      if (ni < 0) continue;
      if (tmp + dpc[ni + 1] < k) {
	tmp += dpc[ni + 1];
      } else {
	idx = i;
	break;
      }
    }
    ans += 'a' + idx;
    assert (idx >= 0);
    k -= tmp;
    pos = nxt[pos + 1][idx];
  }
  cout << ans << endl;
}
