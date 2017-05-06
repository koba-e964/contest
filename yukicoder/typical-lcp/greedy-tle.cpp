#include <algorithm>
#include <bitset>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N = 100010;
string s[N];


// Greedy exhaustive search. It should fail with TLE.
int main(void){
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  assert (n <= 100000);
  int tot_len = 0;
  REP(i, 0, n) {
    cin >> s[i];
    tot_len += s[i].length();
  }
  assert (tot_len <= 800000);
  int m;
  cin >> m;
  assert (m <= 3000000);
  ll x, d;
  cin >> x >> d;
  ll lim = ll(n) * ll(n - 1);
  assert (0 <= x);
  assert (x < lim);
  assert (d < lim);
  assert (0 <= d);
  ll total = 0;
  REP(loop_cnt, 0, m) {
    int i, j;
    i = (x / (n - 1)) + 1;
    j = (x % (n - 1)) + 1;
    if (i > j) {
      swap(i, j);
    } else {
      j++;
    }
    assert (1 <= i);
    assert (i < j);
    assert (j <= n);
    i--, j--;
    int pos = 0;
    int lim = min(s[i].length(), s[j].length());
    for (; pos < lim; ++pos) {
      if (s[i][pos] != s[j][pos]) break;
    }
    total += pos;
    x = (x + d) % (ll(n) * ll(n - 1));
  }
  cout << total << "\n";
}
