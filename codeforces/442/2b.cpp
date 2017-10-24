#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string s;
  cin >> s;
  size_t n = s.length();
  VI ac(n + 1, 0), bc(n + 1, 0);
  REP(i, 0, n) {
    ac[i + 1] = ac[i] + (s[i] == 'a' ? 1 : 0);
    bc[i + 1] = bc[i] + (s[i] == 'b' ? 1 : 0);
  }
  int ma = 0;
  REP(i, 0, n + 1) {
    REP(j, i, n + 1) {
      ma = max(ma, ac[i] + bc[j] - bc[i] + ac[n] - ac[j]);
    }
  }
  cout << ma << "\n";
}
