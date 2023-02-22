#include <iomanip>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    vector<string> s(n);
    REP(i, 0, n) cin >> s[i];
    const int inf = 1e7;
    vector<VI> dist(n, VI(n, inf));
    REP(i, 0, n) {
      dist[i][i] = 0;
      REP(j, 0, n) {
        if (s[i][j] == '1') dist[i][j] = 1;
      }
    }
    REP(k, 0, n) REP(i, 0, n) REP(j, 0, n) dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
    bool ok = true;
    REP(i, 0, n) REP(j, 0, n) if (dist[i][j] >= inf || (dist[i][j] % 2 == 1) != (s[i][j] == '1')) {
      ok = false;
    }
    if (ok) {
      cout << "YES\n";
      vector<PI> e;
      REP(i, 0, n) REP(j, i + 1, n) if (s[i][j] == '1') e.push_back(PI(i + 1, j + 1));
      cout << e.size() << "\n";
      for (PI f: e) {
        cout << f.first << " " << f.second << "\n";
      }
    } else {
      cout << "NO\n";
    }
  }
}
