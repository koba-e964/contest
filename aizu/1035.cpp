#include <iostream>
#include <map>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int w, q;
  while (cin >> w >> q && w + q) {
    VI dat(w, 0);
    map<int, PI> tbl;
    REP(i, 0, q) {
      string t;
      cin >> t;
      if (t == "s") {
        int a, b; cin >> a >> b;
        int idx = -1;
        REP(i, 0, w - b + 1) {
          bool ok = 1;
          REP(j, 0, b) if (dat[i+j]!=0)ok=0;
          if (ok) { idx = i; break; }
        }
        if (idx == -1) {
          cout << "impossible" << endl;
        } else {
          cout << idx << endl;
          REP(i, idx, idx + b) dat[i] = 1;
          tbl[a] = PI(idx, idx + b);
        }
      } else {
        int a; cin >> a;
        PI b = tbl[a];
        REP(i, b.first, b.second) dat[i] = 0;
        tbl.erase(a);
      }
    }
    cout << "END\n";
  }
}
