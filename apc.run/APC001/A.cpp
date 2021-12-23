#include <iostream>
#include <set>

using namespace std;

int main() {
  int t;
  cin >> t;
  while (t--) {
    set<int> s;
    int p, q, a, b, c;
    cin >> p >> q >> a >> b >> c;
    while (1) {
      p = (a * p + b) % c;
      q = (a * q + b) % c;
      int v = 2 * s.count(p) + s.count(q);
      if (v == 0) {
        s.insert(p);
        s.insert(q);
        continue;
      }
      string t[4] = {"", "Acom", "Pany", "Draw"};
      cout << t[v] << endl;
      break;
    }
  }
}
