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
  int n, r;
  while (cin >> n >> r) {
    VI a(n);
    REP(i, 0, n) a[i] = i;
    REP(i, 0, r) {
      int c;
      cin >> c;
      VI b(a.begin(), a.begin() + n / 2);
      a = VI(a.begin() + n / 2, a.end());
      int apos = 0, bpos = 0;
      VI d;
      while (apos < a.size() || bpos < b.size()) {
        if (apos < a.size()) {
          int len = min((int) a.size() - apos, c);
          REP(i, 0, len) d.push_back(a[apos + i]);
          apos += len;
        }
        if (bpos < b.size()) {
          int len = min((int) b.size() - bpos, c);
          REP(i, 0, len) d.push_back(b[bpos + i]);
          bpos += len;
        }
      }
      a = d;
    }
    cout << a[n - 1] << endl;
  }
}
