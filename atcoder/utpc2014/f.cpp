#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int main(void) {
  int t;
  cin >> t;
  while (t--) {
    string s;
    cin >> s;
    int inv = 0;
    REP(i, 0, s.length()) {
      REP(j, 0, i) {
	if (s[i] < s[j]) inv++;
      }
    }
    cout << inv % 2 << endl;
  }
}
