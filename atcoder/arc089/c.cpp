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
  int n;
  cin >> n;
  VI x(n + 1), y(n + 1), t(n + 1);
  REP(i, 1, n + 1) {
    cin >> t[i] >> x[i] >> y[i];
  }
  REP(i, 0, n) {
    int dist = abs(x[i] - x[i + 1]) + abs(y[i] - y[i + 1]);
    int dif = t[i + 1] - t[i];
    if (dist <= dif && (dist + dif) % 2 == 0) {
      continue;
    }
    cout << "No\n";
    return 0;
  }
  cout << "Yes\n";
}
