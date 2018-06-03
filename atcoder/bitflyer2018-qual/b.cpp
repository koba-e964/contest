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
  ll a, b, n;
  cin >> a >> b >> n;
  string x;
  cin >> x;
  REP(i, 0, n) {
    if (x[i] == 'S') {
      a = max(a - 1, 0LL);
    }
    if (x[i] == 'C') {
      b = max(b - 1, 0LL);
    }
    if (x[i] == 'E') {
      if (a == 0 && b == 0) continue;
      if (a >= b) {
        a--;
      } else {
        b--;
      }
    }
  }
  cout << a << endl << b << endl;
}
