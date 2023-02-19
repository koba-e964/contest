#include <algorithm>
#include <iomanip>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  REP(_, 0, t) {
    int n;
    cin >> n;
    VI b(n + 1);
    REP(i, 0, n) {
      int tmp;
      cin >> tmp;
      b[i + 1] = b[i] ^ tmp;
    }
    if (b[n] != 0) {
      cout << "YES\n2\n";
      cout << "1 1\n2 " << n << "\n";
      continue;
    }
    vector<PI> mp;
    REP(i, 1, n) {
      if (mp.size() >= 2) break;
      if (b[i] == 0) continue;
      bool ok = true;
      REP(j, 0, mp.size()) {
        if (mp[j].first == b[i]) {
          ok = false;
          break;
        }
      }
      if (ok) {
        mp.push_back(PI(b[i], i));
      }
    }
    if (mp.size() <= 1) {
      cout << "NO\n";
      continue;
    }
    cout << "YES\n3\n";
    cout << "1 " << mp[0].second << "\n";
    cout << mp[0].second + 1 << " " << mp[1].second << "\n";
    cout << mp[1].second + 1 << " " << n << "\n";
  }
}
