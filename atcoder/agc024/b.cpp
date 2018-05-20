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
  VI p(n);
  REP(i, 0, n) {
    cin >> p[i];
    p[i]--;
  }
  VI inv(n);
  REP(i, 0, n) inv[p[i]] = i;
  int len = 0;
  int cur = 0;
  REP(i, 1, n) {
    if (inv[i - 1] > inv[i]) {
      len = max(len, i - cur);
      cur = i;
    }
  }
  len = max(len, n - cur);
  cout << n - len << endl;
}
