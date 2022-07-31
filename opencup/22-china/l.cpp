#include <algorithm>
#include <cassert>
#include <cctype>
#include <iomanip>
#include <iostream>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    int n;
    string s;
    cin >> n >> s;
    vector<bool> ans(n);
    REP(i, 1, n + 1) {
      int l = i & -i;
      int v = i - 1;
      int cnt = 0;
      while (v > i - l) {
        if (s[v - 1] == '1') cnt++;
        v &= v - 1;
      }
      if ((s[i - 1] == '1' && cnt == 0) || (s[i - 1] == '0' && cnt == 1)) ans[i - 1] = true;
    }
    int res = 0;
    for (bool v: ans) res += v;
    cout << res << "\n";
  }
}
