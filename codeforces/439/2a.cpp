#include <algorithm>
#include <cassert>
#include <cctype>
#include <iostream>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI x(n), y(n);
  set<int> cand;
  REP(i, 0, n) {
    cin >> x[i];
    cand.insert(x[i]);
  }
  REP(i, 0, n) {
    cin >> y[i];
    cand.insert(y[i]);
  }
  int tot = 0;
  REP(i, 0, n) {
    REP(j, 0, n) {
      if (cand.count(x[i] ^ y[j])) {
	tot += 1;
      }
    }
  }
  cout << (tot % 2 == 0 ? "Karen" : "Koyomi") << "\n";
}
