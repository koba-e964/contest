#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
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
  int n;
  cin >> n;
  vector<string> t(n);
  vector<VI> x(n);
  int len = 0;
  REP(i, 0, n) {
    cin >> t[i];
    int k;
    cin >> k;
    x[i] = VI(k);
    REP(j, 0, k) {
      cin >> x[i][j];
      x[i][j]--;
      len = max(len, (int) t[i].length() + x[i][j]);
    }
  }
  string ret(len, 'a');
  REP(i, 0, n) {
    int cur = 0;
    int l = t[i].length();
    REP(j, 0, x[i].size()) {
      REP(k, max(cur, x[i][j]), x[i][j] + l) {
	ret[k] = t[i][k - x[i][j]];
      }
      cur = x[i][j] + l;
    }
  }
  cout << ret << "\n";
}
