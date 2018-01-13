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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

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
  int ma = 0;
  int mi = 0;
  int cur = 0;
  REP(i, 0, s.length()) {
    if (s[i] == '2') {
      cur += 1;
    } else {
      cur -= 1;
    }
    ma = max(ma, cur);
    mi = min(mi, cur);
  }
  if (cur != 0 || mi < 0) {
    cout << -1 << endl;
    return 0;
  }
  cout << ma << endl;
}
