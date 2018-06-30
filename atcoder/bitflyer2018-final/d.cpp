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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

void tapris(VL &x) {
  int n = x.size();
  int row = 0;
  REP(i, 0, 60) {
    int idx = -1;
    REP(j, row, n) {
      if (x[j] & 1LL << i) {
        idx = j;
        break;
      }
    }
    if (idx == -1) continue;
    swap(x[row], x[idx]);
    REP(j, 0, n) {
      if (j == row) continue;
      if (x[j] & 1LL << i) {
        x[j] ^= x[row];
      }
    }
    row++;
  }
  if (DEBUG) {
    REP(i, 0, n) {
      cerr << " " << x[i];
    }
    cerr << endl;
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n), b(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> b[i];
  tapris(a);
  tapris(b);
  cout << (a == b ? "Yes" : "No") << endl;
}
